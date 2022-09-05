use std::io;
use rand::Rng; //trait
use std::cmp::Ordering;

fn main() {
    println!("猜测一个数");

    let sec_num = rand::thread_rng().gen_range(1,101);

    //print!("{}",sec_num);

    loop{
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取");
    
        //let guess:u32 = match guess.trim().parse().expect("请输入一个数字");//expect只接收可处理的异常，
                                                                            // 无法处理的异常直接崩溃
        let guess:u32 = match guess.trim().parse(){//parse会返回ok(类型)和Err()
            Ok(num) => num,
            Err()   => continue,
        };
    
        print!("the num is {}",guess);
    
        match guess.cmp(&sec_num) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {println!("等于");break;}
        }
    }
}
