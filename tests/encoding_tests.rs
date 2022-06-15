use enigma::encoding::Encoder; 
use std::collections::HashMap; 

fn create_trivial() -> Encoder {
    let wires = HashMap::new();  
    let mut cog1: HashMap<char, char> = HashMap::new(); 
    for x in b'a'..=b'z' {
        cog1.insert(x as char, x as char); 
    }
    let cog2 = cog1.clone(); 
    let cog3 = cog1.clone(); 

    Encoder::new(cog1, cog2, cog3, wires)
}
fn create_random() -> Encoder {

}

#[test]
fn no_mutate_1() {
    let encoder = create_trivial(); 
    let str = String::from("Hello world"); 
    assert_eq!(encoder.encode(&str), String::from("Hello world")); 
}

fn no_mutate_2() {
    let encoder = create_trivial(); 
    let str = String::from("blah blah blah"); 
    assert_eq!(encoder.encode(&str), String::from("blah blah blah")); 
}

fn same_output() {
    let encoder = create_trivial(); 
    let str = String::from("Hello world"); 
    assert_eq!(encoder.encode(&str), encoder.encode(&str)); 
}