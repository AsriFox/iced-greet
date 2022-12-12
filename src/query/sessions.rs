fn os_err(err: std::io::Error) -> i32 {
    err.raw_os_error().unwrap_or(2)
}

fn parse_session_file(path: String) -> Result<String, i32> {
    let cat = std::process::Command::new("cat")
        .arg(path)
        .output()
        .expect("Failed to execute 'cat'");

    return if cat.status.success() {
        for line in String::from_utf8(cat.stdout)
            .expect("'stdout' was not a UTF-8 string")
            .lines() 
        {
            if line.contains("Exec") {
                let exec = line.split('=').last();
                if let Some(cmd) = exec {
                    if cmd.len() > 0 {
                        return Ok(cmd.to_string());
                    }
                } 
            }
        }
        Ok(String::new())
    } else {
        Err(
            cat.status.code().unwrap()
        )
    }
}

fn dir_ls(path: &str) -> Result<Vec<String>, i32> {
    let dir = std::fs::read_dir(path);
    if let Err(err) = dir {
        return Err(os_err(err));
    }
    let mut entries = vec![];
    for entry in dir.unwrap() {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                let path = path.to_str();
                if let Some(path) = path {
                    entries.push(path.to_string());
                }
            }
            Err(err) => return Err(os_err(err)),
        }
    }
    return Ok(entries);
}

/// Query xorg sessions from `/usr/share/xsessions`.
/// 
/// Returns either a list of paths to `[session].desktop` files or an `i32` error code.
/// 
/// Typical error codes:
/// 
/// | Code | Name | Error message |
/// |-|-|-|
/// |  1 | EPERM  | Operation not permitted |
/// |  2 | ENOENT | No such file or directory |
/// | 13 | EACCES | Permission denied |
pub fn query_sessions_xorg() -> Result<Vec<String>, i32> {
    match dir_ls("/usr/share/xsessions") {
        Ok(paths) => {
            let mut cmds = vec![];
            for path in paths {
                match parse_session_file(path) {
                    Ok(cmd) => {
                        let cmd = format!(
                            "startx $HOME/.xinitrc {cmd} --",
                        );
                        cmds.push(cmd);
                    }
                    Err(err) => return Err(err),
                }
            }
            Ok(cmds)
        }
        err => err,
    }
}

/// Query Wayland sessions from `/usr/share/wayland-sessions`.
/// 
/// Returns either a list of paths to `[session].desktop` files or an `i32` error code.
/// 
/// Typical error codes:
/// 
/// | Code | Name | Error message |
/// |-|-|-|
/// |  1 | EPERM  | Operation not permitted |
/// |  2 | ENOENT | No such file or directory |
/// | 13 | EACCES | Permission denied |
pub fn query_sessions_wayland() -> Result<Vec<String>, i32> {
    match dir_ls("/usr/share/wayland-sessions") {
        Ok(paths) => {
            let mut cmds = vec![];
            for path in paths {
                match parse_session_file(path) {
                    Ok(cmd) => cmds.push(cmd),
                    Err(err) => return Err(err),
                }
            }
            Ok(cmds)
        }
        err => err,
    }
}
