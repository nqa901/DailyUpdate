//切片
//tip：字符串切片的类型标识是 &str

fn main(){
    //字符串切片

    // let s = String::from("hello world");

    // let hello = &s[0..5];
    // println!("{}",hello);

    // let world = &s[6..11];
    // println!("{}",world);

    // let slice = &s[..2];
    // println!("{}",slice);


    //UTF-8编码中，一个中文占3个字节
    let s = "中国人";
    let a = &s[0..3];
    println!("{}",a);


    //其他切片
    let a1 = [1, 2, 3, 4, 5];

    let slice1 = &a1[1..3];

    assert_eq!(slice1, &[2, 3]);

    println!("{:?}",slice1);

}