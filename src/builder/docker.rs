use std::process::Command;

// Command::new("git").arg("clone").arg("your repo addr")
// Command::new("docker").arg("--rm").arg("--tag").arg("your target image tag").arg(".")

pub fn run_command() {
    let output = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
}
