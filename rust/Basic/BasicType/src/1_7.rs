//rust 的字符 包括各种文字，甚至表情包

fn main(){
    //字符类型占用四个字节

    let x1 = '中';
    let x2 = '😻';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x1));
    println!("字符'😻'占用了{}字节的内存大小",std::mem::size_of_val(&x2));
    
}