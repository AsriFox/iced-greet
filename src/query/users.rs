use std::process::{ Command, ExitStatus };

const ID_MIN: i32 = 1000;
const ID_MAX: i32 = 60000;

pub fn query_usernames() -> Result<Vec<String>, i32> {
    let usr_query = format!("'{{ if ( $3 >= {ID_MIN} && $3 <= {ID_MAX} ) print $1}}'");
    let awk = std::process::Command::new("awk")
        .arg("-F:")
        .arg(usr_query)
        .arg("/etc/passwd")
        .output()
        .expect("Failed to execute 'awk'");

    return if awk.status.success() {
        Ok(
            String::from_utf8(awk.stdout)
                .expect("'stdout' was not a UTF-8 string")
                .split_ascii_whitespace()
                .map(|s| String::from(s))
                .collect()
        )
    } else {
        Err(
            awk.status.code().unwrap()
        )
    }
}