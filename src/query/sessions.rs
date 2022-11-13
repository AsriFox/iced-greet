fn dir_ls(path: &str) -> Result<Vec<String>, i32> {
    let dir = std::fs::read_dir(path);
    return if let Ok(paths) = dir {
        Ok(
            paths.map(
                |entry| String::from(
                    entry.unwrap().path()
                        .to_str().unwrap()
                )
            ).collect()
        )
    } else {
        Err(
            dir.expect_err("Read contents of '/usr/share/xsessions'")
                .raw_os_error().unwrap()
        )
    }
}

pub fn query_sessions_xorg() -> Result<Vec<String>, i32> {
    dir_ls("/usr/share/xsessions")
}

pub fn query_sessions_wayland() -> Result<Vec<String>, i32> {
    dir_ls("/usr/share/wayland-sessions")
}