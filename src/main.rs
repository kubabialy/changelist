use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(&["log", "--pretty=format:%s", "--abbrev"])
        .output()
        .unwrap();
    
    println!("{}", String::from_utf8(output.stdout).unwrap());
}
