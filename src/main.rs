extern crate regex;

fn main() {
	println!("!!!!");
	let input = "(begin (define r 10) (* pi (* r r)))";
	let parsed_code = parse(input);
	println!("{}", parsed_code);
	// Execute script from the command line
}

fn parse(code: &str) -> String {
	// code.to_string()
	let left_paren_re = regex::Regex::new(r"\s*\(\s*").unwrap();
	let right_paren_re = regex::Regex::new(r"\s*\)\s*").unwrap();
	let partial_parsed_code = left_paren_re.replace_all(&code, " ( ");
	let parsed_code = right_paren_re.replace_all(&partial_parsed_code," ) ");
	// let parsed_code = code.replace(" (", "(").replace("( ", "(").replace("(", " ( ").replace()
	let final_string : String = parsed_code.split(" ").collect();
	final_string.clone()
}

// fn tokenize() {

// }

// fn read_from_tokens() {

// }

#[test]
