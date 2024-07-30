pub fn remvove_semicolon(cmd: &str) -> String {
    let cmd = cmd.to_string();

    let end = cmd
        .clone()
        .chars()
        .into_iter()
        .enumerate()
        .find(|(_, x)| *x == ';')
        .unwrap()
        .0;

    cmd[0..end].trim().to_string()
}
 