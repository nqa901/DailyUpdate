//String 类型 与 &str 类型

//字符串字面量是切片

/*该切片指向了程序可执行文件中的某个点，
  这也是为什么字符串字面量是不可变的，
  因为 &str 是一个不可变引用。
*/

//字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间
//String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码。
//字符串中的字符所占的字节数是变化的(1 - 4)

fn main (){
    //String 与 &str 的转换

    //String 转 &str
    let s = String::from("hello,world!");
    let s1 = &s;
    let s2 = s.as_str();
    println!("{}",s1);  
    println!("{}",s2);

    //&str 转 String
    let s1 = "hello,world!";
    let s2 = s1.to_string();


    //追加（push）

    //使用 push() 方法追加字符 char，
    //也可以使用 push_str() 方法追加字符串字面量。

    //在原有的字符串上追加，并不会返回新的字符串
    //该字符串必须是可变的

    let mut s = String::from("Hello ");

    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);

    s.push('!');
    println!("追加字符 push() -> {}", s);


    //插入（insert）
    //使用 insert() 方法在字符串的指定位置插入字符 char，

    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);


    //替换（replace）
    //该方法是返回一个新的字符串，而不是操作原来的字符串。
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");


    //replacen()
    // let string_replace = "I like rust. Learning rust is my favorite!";
    // let new_string_replacen = string_replace.replacen("rust", "RUST", 1);


    //replace_range()
    //该方法仅适用于 String 类型
    //该方法是直接操作原来的字符串，不会返回新的字符串

    // let mut string_replace_range = String::from("I like rust!");
    // string_replace_range.replace_range(7..8, "R");


    //删除 (Delete)
    /*与字符串删除相关的方法有 4 个，
    分别是 pop()，remove()，truncate()，clear()。
    这四个方法仅适用于 String 类型。*/


    //pop(),删除并返回字符串的最后一个字符   存在返回值
    //直接操作原来的字符串
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    dbg!(p1);


    //remove(),删除并返回字符串中指定位置的字符  存在返回值
    //直接操作原来的字符串
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);


    //truncate(),删除字符串中指定位置之后的所有字符
    //直接操作原来的字符串 无返回值
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);


    //clear(),清空字符串
    //直接操作原来的字符串 无返回值
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);


    //Concatenate(连接)
    //要求右边的参数必须为字符串的切片引用（Slice）类型
    //+ 是返回一个新的字符串，所以变量声明可以不需要 mut 关键字修饰。


    //使用 + 运算符连接字符串
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
    result += "!!!";

    println!("连接字符串 + -> {}", result);




    //使用 format! 宏连接字符串
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);


    //字符串转义
    

}