fn main() {
    println!("Hello, world!");
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{s}");

    let _x = 5;
    let _y = _x;


    println!("{_x}");
    let s1 = String::from("Hello");
    println!("{s1}");
    let s2 = s1;

    println!("{s2}");

    let ss1 = String::from("hello");
    let ss2 = ss1.clone();

    println!("ss1 = {}, ss2 = {}", ss1, ss2);
    let z: i32 = 23;
    let s = String::from("Hola qué tal?");
    takes_ownership(s);
    makes_copy(z);
    println!("{z}");
    let mut owned = gives_ownership();
    println!("{owned}");
    owned = takes_and_gives_back(owned);
    println!("{owned}");
    let word = String::from("Banana");
    // let mut len: usize = 0;
    let (word, len) = calculate_length(word);
    println!("{word} has length {len}");
    let word_2 = String::from("wpoeipqoewi");
    println!("{}", calculate_length_2(&word_2));
    println!("{}", word_2);
    let mut hello = String::from("Hello");
    change(&mut hello);
    println!("{}", hello);
    println!("{}", no_dangle());
    println!("{}", first_word(&no_dangle()));

    let salute = String::from("Helloalskdj lasadasdasdasdskjdqpewoi asdasd");
    let hello2 = first_word(&salute);
    println!("{}", hello2);
    println!("{}", rest_words(&salute));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn rest_words(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[(i+1)..s.len()];
        }
    }
    ""
}

fn no_dangle() -> String {
    let s = String::from("Buenos días amigo");
    s
}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    println!("{some_string}");
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");

}
