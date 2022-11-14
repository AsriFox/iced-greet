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

pub fn query_sessions_xorg() -> Result<Vec<String>, i32> {
    dir_ls("/usr/share/xsessions")
}

pub fn query_sessions_wayland() -> Result<Vec<String>, i32> {
    dir_ls("/usr/share/wayland-sessions")
}