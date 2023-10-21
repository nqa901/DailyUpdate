//函数

fn main(){
    never_return();
}

//发散函数
fn never_return() -> !{
    panic!("This function never returns!");
}