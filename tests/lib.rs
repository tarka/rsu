
use docker_test as dt;


#[test]
fn help() {
    let container = dt::setup("usu", None, None, "1.64.0").unwrap();
    let out = container.exec(vec![container.dest_str(), "--help"]).unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(out.status.success());
    assert!(stdout.contains("Run commands as a user"));
}

#[test]
fn not_root() {
    let container = dt::setup("usu", None, None, "1.64.0").unwrap();
    let out = container.exec_as(dt::TESTUSER, vec![container.dest_str(), "/bin/ls"]).unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8(out.stderr).unwrap()
            .contains("Error: Not running as root"));
}
