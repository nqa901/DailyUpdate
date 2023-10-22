//引用

fn main(){

    //一个普通的基本类型引用和解引用的例子
    // let x = 5;
    // let y = &x;
    // println!("x = {}, *y = {}",x,*y);


    //不可变引用
    //&允许使用值，但是不获取所有权

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);


    //可变引用
    //&mut允许修改值
    //同一作用域，特定数据只能有一个可变引用,避免数据竞争
    //可变引用不能与不可变引用同时存在


    let mut s = String::from("Hello");
    let r1 = &mut s;
    r1.push_str(", world!");
    println!("{}",*r1);


    //ref
    //使用ref关键字可以创建引用的引用
    let s2 = String::from("🌟");
    let ref r1 = s2;
    println!("{}",*r1);


    //rust编译器确保引用绝对不会变成悬垂状态
    //编译器可以确保数据不会在引用结束前被释放，要想释放数据，必须先停止其引用的使用.
}


fn calculate_length(s: &String) -> usize {
    s.len()
}