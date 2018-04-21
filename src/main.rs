extern crate tesseract;

use tesseract::ocr;
use std::process::Command;



fn main() {
	let status = Command::new("tesseract")
	                     .args(&["test3.jpg", "output","-l","eng+swe"])
						 .arg("output")
	                     .status()
	                     .expect("failed to execute process");

	println!("process exited with: {}", status);
	assert!(status.success());
}
