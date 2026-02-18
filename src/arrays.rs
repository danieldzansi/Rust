pub fn array (){
    let names =["daniel" , "kofi" , "ama" , "adjoa" , "anita"];

    println!("The first name is {}" , names[0]);

    {
        let mut names =["daniel" , "kofi" , "ama" , "adjoa" , "anita"];
        names [0]= "dzansi";
        println!("the first name is {}" , names[0]);
    }
}