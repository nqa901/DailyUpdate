//区分语句和表达式，表达式不能含分号

fn main(){
    let mut a = 1;
    let b = {
        a += 2;
        a
    };
    assert_eq!(b,3);
}