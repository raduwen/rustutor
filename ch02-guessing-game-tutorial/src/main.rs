use std::io::{self, Write};
use std::cmp::Ordering;
use rand::{thread_rng, Rng};

fn main() {
	println!("Guess the number!");

	// 1〜100までの数字をランダムに生成
	// Rngをスコープに組み込んだ特性でgen_range()が使えるようになる
	// let secret_number = rand::thread_rng().gen_range(1..101); // 上限は排他
	let secret_number = thread_rng().gen_range(1..=100);

	println!("The secret number is: {}", secret_number);

	loop {
		println!("Please input your guess.");
		let mut guess = String::new();

		print!("> ");
		io::stdout().flush().unwrap();
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please type a number!");
				continue;
			},
		};
		println!("You guessed: {}", guess);

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			},
		}
	}
}
