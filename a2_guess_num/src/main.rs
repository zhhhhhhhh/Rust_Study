use std::io;
use rand::Rng; //trait
use std::cmp::Ordering;

fn main() {
    println!("猜测一个数");

    let secNum = rand::thread_rng().gen_range(1,101);

    print!("{}",secNum);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取");

    let guess:u32 = guess.trim().parse().expect("请输入一个数字");

    print!("the num is {}",guess);

    match guess.cmp(&secNum) {
        Ordering::Less => println!("太小了"),
        Ordering::Greater => println!("太大了"),
        Ordering::Equal => println!("等于"),
    }
}
