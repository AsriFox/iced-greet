fn dir_ls(path: &str) -> Result<Vec<String>, i32> {
    let dir = std::fs::read_dir(path);
    return if let Ok(paths) = dir {
        Ok(
            paths.map(
                |entry| { 
                    // let path = entry.unwrap().path().to_str().unwrap();
                    // let contents = String::from_utf8_lossy(
                    //     &std::fs::read(path).unwrap()
                    // );
                    String::from(entry.unwrap().path().to_str().unwrap())
                    // , String::from(contents) )
                }
            ).collect()
        )
    } else {
        Err(
            dir.expect_err(
                format!("Read contents of '{path}'").as_str()
            )
            .raw_os_error().unwrap()
        )
    }
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
    dir_ls("/usr/share/xsessions")
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
    dir_ls("/usr/share/wayland-sessions")
}