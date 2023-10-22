//完成String 和 &str 之间的转换的几种常规方法
/*
例题原代码：
    let x = String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
*/

fn main(){
    //方法一  对String类型调用clone方法
    // let x = String::from("hello, world");
    // let y = x.clone();
    // println!("{},{}",x,y);


    //方法二  使用&str类型
    // let x = "hello, world";
    // let y = x;
    // println!("{},{}",x,y);


    //方法三  使用&T不可变引用类型
    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);


    //方法四  使用to_str方法
    // let x = String::from("hello, world");
    // let y = x.to_str();
    // println!("{},{}",x,y);
}