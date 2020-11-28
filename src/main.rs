use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(&["log -n 5"])
        .output()
        .unwrap();
    
    println!("{}", String::from_utf8(output.stdout).unwrap());
}
