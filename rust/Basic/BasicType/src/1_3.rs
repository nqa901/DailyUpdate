//定义常量以及变量遮蔽

//常量名全大写，常量名以及取值均可以用下划线分隔
const MAX_POINTS: u32 = 100_000;

fn main(){
    //关键在于观察变量的作用域
    let x = 5;
    let x = x + 1;
    println!("The value of x is:{}",x);
    println!("The value of MAX_POINTS is:{}",MAX_POINTS);
}