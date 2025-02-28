// use core::arch;

fn main() {
    println!("Hello world!");

    another_function(5, 'h');//実引数
    mikoti_function("Mikoti", 100000);
}

fn another_function(value: i32, unit_label: char) {//仮引数
    println!("The measurement is: {}{}", value, unit_label);
}

fn mikoti_function(she: &str, zenrosu: i32){
    println!("{} is so pon cuz {} times she lost her eritora",she, zenrosu);
}