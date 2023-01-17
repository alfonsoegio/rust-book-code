use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    println!("Hello, world!");
    // let mut v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v2: Vec<i32> = Vec::new();
    v2.push(5);
    v2.push(5);
    v2.push(6);
    v2.push(6);
    v2.push(7);
    v2.push(7);

    println!("{:?}", v);
    println!("{:?}", v2);

    let third: &i32 = &v2[2];
    let thirdo: Option<&i32> = v2.get(6);
    match thirdo {
        Some(thirdo) => println!("The third of v2 is {}", thirdo),
        None => println!("There's no thirdo element")
    }
    println!("{}", third);
    println!("---------------------");
    for i in &v2 {
        println!("{i}");
    }
    for i in &mut v2 {
        *i += 40;
    }
    for i in &v2 {
        println!("{i}");
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(29.29),
        SpreadsheetCell::Text(String::from("blue"))
    ];
    let hello = String::from("안녕하세요");
    println!("{hello}");
    println!("Row is {:?}", row);

    let mut s = String::from("foo");
    s.push_str(" bar");
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;
    println!("{s3}");
    println!("{s2}");

    let hello = String::from("Hello");
    let s4 = format!("{hello}-{s2}-{s3}");
    println!("{hello}");
    println!("{s4}");

    let hellor = "Здравствуйте";
    let answer = &hellor[18..20];
    println!("{answer}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    let team_name = String::from("Yellow");
    let score = scores.get(&team_name); //.copied().unwrap_or(0);
    println!("{:?}", score);

    match score {
        Some(score) => println!("Score is {}", score),
        None => println!("No score recorded")
    };

    for (key, value) in &scores {
        println!("{key}\t{value}");
    }
    scores.insert(String::from("Yellow"), 3);
    scores.entry(String::from("Yellowish")).or_insert(50);
    println!("{:?}", scores);

    let text = "Hello world wonderful world";
    let mut tmap = HashMap::new();
    for word in text.split_whitespace() {
        let count = tmap.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", tmap);
}
