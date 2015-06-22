use std::process::Command;
fn main() {
    let status = Command::new("ruby").arg("test.rb").status().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    println!("process exited with: {}", status);
    let status = Command::new("python").arg("test.py").status().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
}
