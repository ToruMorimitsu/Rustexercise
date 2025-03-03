// use core::arch;

use core::num;

fn main() {
    println!("Hello world!");

    another_function(5, 'h');//実引数
    mikoti_function("Mikoti", 100000);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
// =====================================================
    
    //ブールのデモンストレーション

    let number = 7;

    if number < 5{
        println!("condition was true");
    }else{
        println!("condition was false");
    }
// =====================================================

    // let number = 3;
    // if number {
    //     println!("number was three");
    // }

    let number = 3;
    if number != 0{//ここ条件式にできるんや
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0{
        println!("number is divisible by 4");
    }else if number % 3 == 0{
        println!("number is divisible by 3");
    }else if number % 2 == 0{
        println!("number is divisible by 2");
    }else{
        println!("number is not divisible by 4, 3, or 2");
    }
//else if 文を使いすぎるのはあんまり綺麗ではないらしい。
//これらのケースには強力なrustの枝分かれ構文、matchが有用らしい。

    let condition = true;
    let number = if condition {5} else {6};//型を合わせる。

    println!("The value of number is {}", number);
// =====================================================
//loopでコードを繰り返す

    loop {
        println!("again!");
        break;
    }

    let mut count = 0;
    'counting_up: loop{//counting_upラベル
        println!("count = {}", count);
        let mut remaining = 10;

        loop{
            println!("remaining = {}", remaining);
            if remaining == 9{
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

//whileで条件付きループ

    let mut number = 3;

    while number != 0{
        println!("{}!", number);

        number -=1;
    }

    println!("LIFTOFF!!!!");

//revメソッドを使用したループ

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!!!");

//forでコレクションを覗き見る

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index <5 {
        //値は{}です
        println!("The alue is: {}", a[index]);

        index += 1;
    }
//↑これは間違いが発生しやすく、かつ遅い
//改善策としてはforループを使ってコレクションの各アイテムに対してコードを
//実行できるこれがいい
//rustで使用頻度が最も高い

    let a = [10, 20, 30, 40, 50];

    for element in a{
        println!("The value is: {}", element);
    }

// =====================================================


}

fn another_function(value: i32, unit_label: char) {//仮引数
    println!("The measurement is: {}{}", value, unit_label);
}

fn mikoti_function(she: &str, zenrosu: i32){
    println!("{} is so pon cuz {} times she lost her eritora",she, zenrosu);
}

fn five() -> i32{
    5
}

fn plus_one(x :i32) -> i32 {
    x + 1
}
