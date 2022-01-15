
mod util;

use crate::util::*;


#[test]
fn help() {
    let container = setup().unwrap();
    let out = container.exec(vec![INST_BIN, "--help"]).unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    println!("OUT = {}", stdout);
    println!("ERR = {}", String::from_utf8(out.stderr).unwrap());
    assert!(out.status.success());
    assert!(stdout.contains("Run commands as a user"));
}
