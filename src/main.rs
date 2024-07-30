// #![allow(dead_code)]

use std::process::exit;

use formatting::remvove_semicolon;
use regex::Regex;
use table::Table;
mod formatting;
mod table;

fn main() {
    let mut buf;

    loop {
        buf = rprompt::prompt_reply(">> ").unwrap();
        parse_cmd(buf.as_str());
    }
}

fn exit_cmd() {
    println!("Exiting with code: 0");
    exit(0);
}

fn parse_cmd(cmd: &str) {
    if cmd.eq(".exit") {
        exit_cmd()
    }

    let create_regex = Regex::new(r"^CREATE TABLE\b.*;$").unwrap();
    if create_regex.is_match(cmd) {
        let cmd = cmd.split("CREATE TABLE ").nth(1).unwrap();
        let cmd = cmd.split("\r\n").nth(0).unwrap();
        match Table::new(remvove_semicolon(cmd)) {
            Ok(t) => println!("{:#?}", t),
            Err(e) => println!("{e}"),
        };
    } else {
        println!("{:#?}", cmd);
    }
}
