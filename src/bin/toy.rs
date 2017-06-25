// This is a minimalist model
// 这是个小的玩具模型

fn main() {
    println!("{}", run("23").unwrap());
}

enum Token {
	Left,
	Right,
	Num(usize),
	Keyword(Key),
	Unknown,
}

enum Key {
	Add,
	Sum,
	Mul,
	Div,
	Unknown,
}

fn run(expression: &str) -> Option<usize> {
    let mut token: Vec<Token> = get_token(expression);
	token = solution(token);
	if token.len() == 1 {
		let foo = token.pop();
		match foo {
			Some(Token::Num(x)) => return Some(x),
			_ => return None,
		}
	}
    None
}

fn solution(token: Vec<Token>) -> Vec<Token>  {
	token
}

fn get_token(expression: &str) -> Vec<Token> {
	let mut token: Vec<Token> = Vec::new();
	let mut el_num: usize = 0;
	let mut el_key: String = "".to_string();
	let mut el_flag: Token = Token::Num(0);
    for el in expression.chars() {
	    match el {
		    '(' => {
				match el_flag {
					Token::Unknown => {},
					_ =>{},
				}
				token.push(Token::Left);
				el_flag = Token::Unknown;
			},
			')' => token.push(Token::Right),
			e @ '0'...'9' => el_num = el_num * 10 + e as usize - 48,
			' ' => {},
			_ => el_key += format!("{}", el).as_str(),
		}
	}
	match el_flag {
		Token::Num(_) => token.push(Token::Num(el_num)),
		_ => token.push(Token::Unknown),
	}
	token
}

fn get_key(expression: &str) -> Key {
	match expression {
		"+" | "Add" => Key::Add,
		"-" | "Sum" => Key::Sum,
		"*" | "Mul" => Key::Mul,
		"/" | "Div" => Key::Div,
		_ => Key::Unknown,
	}
}
