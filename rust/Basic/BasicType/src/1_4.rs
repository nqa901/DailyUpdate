//数值类型


//整型默认i32
//浮点型默认f64


fn main(){
    //补码循环溢出
    let a : u8 = 255;
    let b = a.wrapping_add(10);
    println!("The value of b is:{}",b);


    //控制数字位数
    let array = [420.0, 56_f32, 784.9657_f32];
    println!("{:.2}",array[2]);


    //简单循环
    for i in 0..10{
        println!("{}",i);
    }


    //使用as完成类型转换
    let a :u32 = 1000_u16 as u32;
    println!("{}",a);
}