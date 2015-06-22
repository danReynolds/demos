use std::process::Command;
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        let status = Command::new("ruby").arg("test.rb").status().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
        "hey"
    });
    println!("process exited with: {}", handle.join().unwrap());
    let handle2 = thread::spawn(|| {
        let status = Command::new("python").arg("test.py").status().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
        "hey2"
    });
    println!("process exited with: {}", handle2.join().unwrap());
}
