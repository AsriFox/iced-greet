use std::{ env, os::unix::net::UnixStream };
use greetd_ipc::{ codec::SyncCodec, AuthMessageType, ErrorType, Request, Response };

/// Wrapper class for greetd IPC communication.
/// 
/// Contains a Unix socket that uses greetd_ipc protocol to (maybe) log in.
/// 
/// For more information on response variants see [`LoginResult`].
/// 
/// ## Example
/// ```
/// use iced_greet::greeter::{ Greeter, LoginResult };
/// 
/// let mut greeter = Greeter::new("command/to/run");
/// 
/// let result: LoginResult = greeter.request_login("username");
/// 
/// if let Ok(login_result) = result {
///     let result = match login_result {
///         LoginResult::PromptVisible(_)
///         | LoginResult::PromptSecret(_) => {
///             greeter.respond_to_auth_message("password")
///         },
///         _ => {
///             stderr!("Something went wrong");
///             Err(login_result)
///         }
///     }
///     if let Ok(login_result) = result {
///         match login_result {
///             LoginResult::Failure => {
///                 stderr!("Wrong username or password");
///             },
///             LoginResult::Success => {
///                 stderr!("Login successful");
///             },
///             _ => {
///                 stderr!("Something went wrong")
///             },
///         }
///     }
/// }
/// ```
pub struct Greeter {
    stream: UnixStream,
    pub cmd: String,
}

pub enum LoginResult {
    Success,
    Failure,
    PromptVisible(String),
    PromptSecret(String),
    AuthInfo(String),
    AuthError(String),
}

impl Default for Greeter {
    fn default() -> Self {
        Self::new("startx".to_string())
    }
}

type Result = std::result::Result<LoginResult, Box<dyn std::error::Error>>;

impl Greeter {
    pub fn new(cmd: String) -> Self {
        let path = env::var("GREETD_SOCK")
            .expect("Environment variable GREETD_SOCK is not set");
        let stream = UnixStream::connect(path)
            .expect("Failed to connect Unix socket");
        Self { 
            stream,
            cmd: cmd.clone(),
        }
    }
    
    pub fn request_login(&mut self, username: String) -> Result {
        Request::CreateSession { username }
            .write_to(&mut self.stream)?;
        self.read_response()
    }
    
    pub fn respond_to_auth_message(&mut self, response: Option<String>) -> Result {
        Request::PostAuthMessageResponse { response }
            .write_to(&mut self.stream)?;
        self.read_response()
    }

    pub fn cancel_login(&mut self) -> Result {
        Request::CancelSession
            .write_to(&mut self.stream)?;
        self.read_response()
    }
    
    fn read_response(&mut self) -> Result {
        match Response::read_from(&mut self.stream)? {
            Response::AuthMessage {
                auth_message,
                auth_message_type,
            } => {
                Ok(
                    match auth_message_type {
                        AuthMessageType::Visible => LoginResult::PromptVisible(auth_message),
                        AuthMessageType::Secret => LoginResult::PromptSecret(auth_message),
                        AuthMessageType::Info => LoginResult::AuthInfo(auth_message),
                        AuthMessageType::Error => LoginResult::AuthError(auth_message),
                    }
                )
            }
            Response::Success => {
                Request::StartSession {
                    cmd: vec![self.cmd.clone().to_string()],
                }.write_to(&mut self.stream)?;
                
                Ok(LoginResult::Success)
            }
            Response::Error {
                error_type,
                description,
            } => {
                match error_type {
                    ErrorType::AuthError => Ok(LoginResult::Failure),
                    ErrorType::Error => Err(format!("login error: {:?}", description).into()),
                }
            }
        }
    }
}
