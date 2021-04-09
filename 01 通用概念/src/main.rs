fn main() {
    //-------- 变量&常量 ------------------------------
    let x = 5;              //不可变变量
    println!("x:{}", x);
    let mut y = 6;          //可变变量
    println!("y:{}", y);
    y = 7;
    println!("y:{}", y);
    const MAX_VALUE: u32 = 100_00;  //常量
    println!("MAX_VALUE:{}", MAX_VALUE);

    let x = x + 1;          //隐藏不可变的变量，等将x+1的新值赋给新的变量x，隐藏旧的x变量
    println!("x:{}", x);

    let mut space = "   ";
    let space = space.len();    //run出现警告，不能改变变量的值，去掉mut则正常
    println!("space:{}", space);

    //--------- 标量类型 ---------------------------------------------
    //标量类型，整型、浮点型、布尔类型、字符类型
    let ix = 1111;  //类型推断
    let ix2 = 3;
    let fx = 1.0;
    let bx = true;
    let sx = "string";
    println!("ix:{}", ix * ix2); //避免不同类型的计算，编译器会进行自动类型转换，将运算节点转换成相同的类型，可能会出错。

    //-------------- 复合类型 ------------------------------------
    //复合类型，多个值组合的类型。元祖和数组。
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tx = (2.2, 2, 333);
    let (t1, t2, t3) = tx;      //元祖解构
    println!("t1:{} t2:{} t3:{}", t1, t2, t3);
    println!("t1:{} t2:{} t3:{}", tx.0, tx.1, tx.2);

    let array = [1, 2, 3, 4, 5];    //数组的类型必须一样
    let arr_x: [u8; 2] = [2, 2];    //声明固定长度和类型的数组
    println!("x1:{} x2:{}", arr_x[0], arr_x[1]);

    //--------- 函数 --------------------------------------
    other();
    other_x(22, "aaaa");
    let y = {
        let x = 13;
        x + 1
    };      //花括号内的代码块，此表达式最总的返回值是x+1，并赋值给y
    println!("y:{}", y);
    println!("{}+{}={}", 1, 2, plus(1, 2));
    println!("{}-{}={}", 1, 2, sub(1, 2));

    //----- 控制流 -----------------------------------
    let number = 5;
    if number < 5 {
        println!("number lt 5");
    } else {
        println!("number gte 5");
    }

    let nf = if number < 5 { "yes" } else { "no" };
    println!("nf:{}", nf);

    loop {
        println!("ssss");
        break;
    }

    let mut x = 1;
    while x < 3 {
        println!("while x:{}", x);
        x += 1;
    }

    let arr = ["aaa", "bbb", "ccc"];
    for e in arr.iter() {
        println!("arr value:{}", e);
    }

    for number in 1..4 {
        println!("{}!", number);
    }
    
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn other() {
    println!("other fn!!");
}

fn other_x(x: i32, y: &str) {
    println!("other x:{} y:{}", x, y);
}

fn plus(x: i32, y: i32) -> i32 {
    return x + y;       //return xxxx; 是一条表达式，表示返回
}

fn sub(x: i32, y: i32) -> i32 {
    x - y           // 此表达式无分号，直接返回
}