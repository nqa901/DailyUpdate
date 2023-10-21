    //浮点陷阱


    //tip1:浮点数不适合用于测试相等

    fn main(){
        //错误示例:    assert!(0.1 + 0.2 == 0.3);


        /*正确示例1:   使用更容易精度相当的f32
        assert!(0.1 :f32 + 0.2 :f32 == 0.3 :f32); */


        //正确示例2:   使用assert_eq!宏
        assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.001);              
    }
    