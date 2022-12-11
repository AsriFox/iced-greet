pub mod cmd;
pub mod users;
pub mod sessions;

pub fn query_all_cmds() -> Vec<String> {
    let mut sessions = vec![];
    if let Ok(cmds) = cmd::query_cmds() {
        for s in cmds {
            if !sessions.contains(&s) {
                sessions.push(s);
            }
        }
    }
    if let Ok(sessions_wayland) = sessions::query_sessions_wayland() {
        for s in sessions_wayland {
            if !sessions.contains(&s) {
                sessions.push(s);
            }
        }
    }
    if let Ok(sessions_xorg) = sessions::query_sessions_xorg() {
        for s in sessions_xorg {
            if !sessions.contains(&s) {
                sessions.push(s);
            }
        }
    }
    sessions
}
