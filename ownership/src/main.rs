fn main() {
    println!("Hello, world!");

    // let s = String::from("val");
    // takes_ownership(&s);
    // println!("{s}");
    // let i = 13;
    // makes_copy(i);
    // println!("{i}");
    // let s1 = String::from("value");
    // let s2 = s1;
    // println!("{s2}");
    // // println!("{s1}");
    // let mut s = String::from("val");
    // let r1 = &mut s;
    // println!("{r1}");
    // let r2 = &mut s;
    // println!("{r2}");
    // println!("{}, {}", s1, s2);

    let mut s = String::from("va lue");
    let f = first_word(&s);
    println!("{f}");
    s.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}

fn _takes_ownership(some_string: &String) {
    println!("{some_string}");
}

fn _makes_copy(some_int: i32) {
    println!("{some_int}");
}


