
use docker_test::util::build_and_deploy;

const RUST_VERSION: &str = "1.65.0";
const TEST_USER: &str = "nobody";

#[test]
fn help() {
    let (container, bin) = build_and_deploy("usu", None, None, RUST_VERSION).unwrap();
    let out = container.exec(vec![bin.as_str(), "--help"]).unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(out.status.success());
    assert!(stdout.contains("Run commands as a user"));
}

#[test]
fn not_root() {
    let (container, bin) = build_and_deploy("usu", None, None, RUST_VERSION).unwrap();
    let out = container.exec_as(TEST_USER, vec![bin.as_str(), "/bin/ls"]).unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8(out.stderr).unwrap()
            .contains("Error: Not running as root"));
}
