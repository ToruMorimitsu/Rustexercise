fn main(){
    let x = 5;
    let x = x+ 1;
    println!("The value of x is {}", x);


    {
        let x = x * 2;
        println!("The value x inside of bracket is {}", x);
    }
    println!("The value of x is {}", x);
}