//尝试导入num库，哎嘿

use num::complex::Complex;

fn main(){
    let a = Complex{re:1.0, im:2.0};
    let b = Complex::new(3.0, 4.0);
    println!("a + b =  {}", a + b);
}