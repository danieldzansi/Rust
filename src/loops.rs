pub fn lop (){

    let mut count =  1; 
    loop {
        println!("this will continue forever");
         
        if count == 5 {
            break;
        }
        count +=  1
    }

    for i in 3..7 {
         println!("i is {}" , i)
    }

    count = 1 ;

    while count <= 6{
       println!("count {}" , count );
       count += 1 ;
    }
}