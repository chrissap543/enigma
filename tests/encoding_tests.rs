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
    create_trivial()
}

#[test]
fn no_mutate_1() {
    let mut encoder = create_trivial();
    let str = String::from("Hello world");
    assert_eq!(encoder.encode(&str), String::from("Hello world"));
}

#[test]
fn no_mutate_2() {
    let mut encoder = create_trivial();
    let str = String::from("blah blah blah");
    assert_eq!(encoder.encode(&str), String::from("blah blah blah"));
}

#[test]
fn same_output_1() {
    let mut encoder1 = create_trivial();
    let mut encoder2 = encoder1.clone();
    let str = String::from("Hello world");
    assert_eq!(encoder1.encode(&str), encoder2.encode(&str));
}

#[test]
fn same_output_2() {
    let mut encoder1 = create_random();
    let mut encoder2 = encoder1.clone();
    let str = String::from("abcdefgh");
    assert_eq!(encoder1.encode(&str), encoder2.encode(&str));
}
