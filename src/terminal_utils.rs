use std::io::{stdout, Write};

pub fn clear_terminal() {
	print!("\x1B[2J\x1B[1;1H");
	flush();
}

pub fn flush() {
	stdout().flush().unwrap();
}
