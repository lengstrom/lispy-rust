extern crate regex;

fn main() {
	// Execute script from the command line
}

fn parse(code: &str) -> Vec<&str> {
	// code.to_string()
	let leftParenRe = regex::Regex::new(r"\s*\(\s*").unwrap();
	let rightParenRe = regex::Regex::new(r"\s*\)\s*").unwrap();
	let partialParsedCode = leftParenRe.replace_all(code, r" ( ");
	let parsedCode = rightParenRe.replace_all(partialParsedCode.as_slice()," ) ");
	// let parsedCode = code.replace(" (", "(").replace("( ", "(").replace("(", " ( ").replace()
	parsedCode.as_slice().split(" ").collect()
}

fn tokenize() {

}

fn read_from_tokens() {

}

#[test]
fn it_works() {
	let parsedCode : Vec<&str> = parse("(begin (define r 10) (* pi (* r r)))");
	println!("{}", parsedCode);
}
