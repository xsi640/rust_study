use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数字！");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("随机数：{}", secret_number);


    println!("输入数字：");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取数字失败。");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜到：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小"),
            Ordering::Greater => println!("太大"),
            Ordering::Equal => {
                println!("你赢了！");
                break;
            }
        }
    }
}
