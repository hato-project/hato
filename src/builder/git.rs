use std::process::Command;

pub fn clone_repo(repo_url: &String, work_dir: &String) {
    println!("{}", repo_url);

    println!("hey {}", repo_url);

    let output = Command::new("git")
        .current_dir(work_dir)
        .arg("clone")
        .arg(repo_url)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    };
}
