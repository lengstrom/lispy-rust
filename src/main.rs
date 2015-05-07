extern crate regex;

fn main() {
	// Execute script from the command line
}

fn parse(code: &mut str) -> Vec<&str>{
	// code.to_string()
	let leftParenRe = Regex::new("\s*\(\s*").unwrap();
	let rightParenRe = Regex::new("\s*\)\s*").unwrap();
	let parsedCode = rightParenRe.replace_all(leftParenRe.replace_all(code, " ( ")," ) ")
	parsedCode.split(" ")
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
