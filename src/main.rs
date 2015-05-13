extern crate regex;

fn main() {
	println!("!!!!");
	let input = "(begin (define r 10) (* pi (* r r)))";
	let parsed_code = parse(input);
	println!("{}", arr2str(parsed_code));
	// Execute script from the command line
}

fn arr2str(a:Vec<&str>) -> &str {
	let mut s = "[";
	for i in a.iter() {
		s = concat!(s, i);
	}
	s = concat!(s, " ]");
	s;
}

fn parse(code: &str) -> Vec<&str> {
	// code.to_string()
	let left_paren_re = regex::Regex::new(r"\s*\(\s*").unwrap();
	let right_paren_re = regex::Regex::new(r"\s*\)\s*").unwrap();
	let partial_parsed_code = left_paren_re.replace_all(&code, " ( ");
	let parsed_code = right_paren_re.replace_all(&partial_parsed_code," ) ");
	// let parsed_code = code.replace(" (", "(").replace("( ", "(").replace("(", " ( ").replace()
	let final_string : Vec<&str> = parsed_code.split(" ").collect();
	final_string.clone()
}

// fn tokenize() {

// }

// fn read_from_tokens() {

// }

// #[test]
