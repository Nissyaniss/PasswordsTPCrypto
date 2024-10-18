pub trait StringExt {
	fn is_ascii_printable(&self) -> bool;
}

pub const ASCIITABLE: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~ ";

impl StringExt for String {
	fn is_ascii_printable(&self) -> bool {
		for c in self.chars() {
			if !ASCIITABLE.contains(c) {
				return false;
			}
		}
		true
	}
}
