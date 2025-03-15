fn main(){
    let s1 = String::from("hello goodbye");
    let s2 = s1.clone();
    println!("s1= {}, s2= {}", s1, s2);
    //ヒープがコピーされる↑

    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    //give stringは戻り値を呼び出した関数にムーブする。
    let some_string = String::from("hello");
    some_string//some stringが返され、呼び出した返り値にムーブされる。
}
//takes_and_gives_back はStringを一つ受け取り、返す
fn takes_and_gives_back(a_string: String) -> String{
     // a_stringがスコープに入る
    a_string// a_stringが返され、呼び出し元関数にムーブされる
}