use std::process::Command;

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

pub fn build_image(image_tagname: String) {
    let output = Command::new("docker")
        .arg("--rm")
        .arg("--tag")
        .arg(image_tagname)
        .arg(".")
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

pub fn push_image(image_tagname: String) {
    let output = Command::new("docker")
        .arg("push")
        .arg(image_tagname)
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
