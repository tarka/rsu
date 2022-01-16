
mod util;

use crate::util::*;


#[test]
fn help() {
    let container = setup().unwrap();
    let out = container.exec(vec![INST_BIN, "--help"]).unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(out.status.success());
    assert!(stdout.contains("Run commands as a user"));
}

#[test]
fn not_root() {
    let container = setup().unwrap();
    let out = container.exec_as(TESTUSER, vec![INST_BIN, "/bin/ls"]).unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8(out.stderr).unwrap()
            .contains("Error: Not running as root"));
}
