pub fn structs(){
    
    struct Person{
 
        name : String ,
        age :i32 ,
        can_vote :bool ,
    }
   


    let user = Person{
        name : String::from("john"),
        age : 45,
        can_vote : true,
    };

    println!("name {}", user.name);
    println!("age {}" , user.age);
    println!("vote {}", user.can_vote);
}

