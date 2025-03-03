use std::io;

fn main (){
    let a = [1,2,3,4,5];
    println!("Please enter an array index.");

    let mut index = String::new();//文字列として

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

    let index : usize = index//整数として
    //usize: 符号なし整数型。ビットを書かなくていい。勝手に判断される。
    
    .trim()
    .parse()
    .expect("Index entered was not a Number");

    let element = a[index];//配列の中にアクセスできる

    println!(
        "The value of the index element at index {} is: {}",
        index, element
    );
}