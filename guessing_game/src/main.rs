use std::io::{self, Read};

fn main() {
//<<<<スコープ
   println!("Guess the number!");

   println!("Please input your guess.");

   let mut guess = String::new();

   io::stdin()//標準入力
   .read_line(&mut guess)//共有参照guessに束縛
   .expect("Failed to read line");//エラーハンドリング

   println!("You guessd: {}", guess);
//>>>>スコープ
}
