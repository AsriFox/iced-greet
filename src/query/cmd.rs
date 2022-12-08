/// Query commands from `/etc/greetd/environments`.
/// 
/// Returns either a list of names or an `i32` error code.
/// 
/// Typical error codes:
/// 
/// | Code | Name | Error message |
/// |-|-|-|
/// |  1 | EPERM  | Operation not permitted |
/// |  2 | ENOENT | No such file or directory |
/// | 13 | EACCES | Permission denied |
pub fn query_cmds() -> Result<Vec<String>, i32> {
    let cat = std::process::Command::new("cat")
        .arg("/etc/greetd/environments")
        .output()
        .expect("Failed to execute 'cat'");

    return if cat.status.success() {
        Ok(
            String::from_utf8(cat.stdout)
                .expect("'stdout' was not a UTF-8 string")
                .lines()
                .map(|s| String::from(s))
                .collect()
        )
    } else {
        Err(
            cat.status.code().unwrap()
        )
    }
}