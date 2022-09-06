const abc:u32 = 123;//静态声明需要指定类型，需要全大写加下划线，否则warning


fn main(){
    println!("hhh");

    //let x = 5;//不可变
    let mut x = 5;//加mut之后可变
    println!("val is {}",abc);
    //shadow

    let y = 6;
    let y = y+1;
    println!("val is {}",y);//类似函数函数重载？（变量重载？）

}