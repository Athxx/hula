use std::io::{stdin, stdout, Write};

pub fn read_input(text: &str) -> String {
	print!("{}", text);
	stdout().flush();
	let mut s = String::new();
	stdin().read_line(&mut s).expect("Did not enter a correct string");
	s.replace('\r', "").replace('\n', "").trim().to_owned()
}

fn read_param() {}
