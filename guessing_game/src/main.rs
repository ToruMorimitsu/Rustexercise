use std::io::{self};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
//<<<<スコープ
   println!("Guess the number!");

   let secret_number = rand::thread_rng().gen_range(1..101);

//    println!("The secret number is {}", secret_number);

   loop {
       println!("Please input your guess.");

        let mut guess = String::new();
            //可変の変数を作成して = 空のstringインスタンス(実体)に束縛(代入)される

        io::stdin()//標準入力
        .read_line(&mut guess)//共有&可変mut参照を変数guessに束縛
        .expect("Failed to read line");//エラーハンドリング

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                //parse(文字列から数列への変換)に成功したらOkに格納したnumの値を返し
                //guessにそれを束縛
                Err(_) => continue,
                //それに失敗したらプログラムを次のループに入らせる
            };
            //.expect("Please type a number!");

        println!("You guessd: {}", guess);

        match guess.cmp(&secret_number){//cmpは2つの値の比較を行う
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
   }
    
//>>>>スコープ
}
