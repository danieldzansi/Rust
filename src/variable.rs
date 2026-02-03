pub fn run (){
    let x: i32=8;
    let y: i32=9;
    
    assert_eq!(x + y, 17);
    println!("success");
    println!("jonn is {} years old",x)
}

pub fn runn (){
    let x :i32 =5;
    {
        let x =12;
        assert_eq!(x,12);

    }
    assert_eq!(x,5);
    println!("{}",x);
    let x=42;
    println!("{}",x);
}