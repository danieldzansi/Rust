use std::collections::HashMap;
pub fn hashmaps (){
    let mut capital_cities = HashMap::new();

    capital_cities.insert ("ghana", "accra");
    capital_cities.insert ("canada", "toronto");

    println!{"{:?}",capital_cities};
}