fn main() {
    println!("Hello, world!");
    //整数类型i32,u32，占用32位空间，，8、16、32、64、128位*有无符号
    //isize\usize,根据计算机决定，32位或64位
    //整数溢出：
    //debug下编译，如果溢出会panic --release下，会值翻转，（u8)256 = 0; (u8)257 = 1;

    //浮点，f32,f64，默认f64
    let abc = 3.12;
    let abc:f32 = 3.14;
    //其他都差不多
}