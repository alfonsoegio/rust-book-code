use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 4;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in three hours!");

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y outside the inner scope is: {y}");

    let spaces = "     ";
    let spaces = spaces.len();

    println!("spaces value is {spaces}");

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("guess value is {guess}");

    let xp = 2.0;

    let yp: f32 = 3.0;

    println!("xp value is {xp}, yp value is {yp}");

    let t = true;
    let f: bool = false;
    println!("t is {t}, and f is {f}");

    let heart_eyed_cat: char = '😻';
    println!("Here it comes ... {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (xx, yy, zz) = tup;

    println!("The value of yy is {yy}");
    println!("The value of xx is {xx}");
    println!("The value of zz is {zz}");

    let result = tup.0;

    println!("The value of xx is {result}");

    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February"];

    let b = [3; 5];

    let a0 = a[0];
    println!("a[0] is {a0}");

    let months1 = months[1];
    println!("months1 is {months1}");

    let b1 = b[0];
    println!("b1 is {b1}");

    println!("Please, enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is {element}");

    another_function(1212);
    print_labeled_measurements(5, 'h');

    let number = 32;

    if number > 5 {
        println!("{number} is greater than 5!");
    } else {
        println!("Impossible!");
    }
}

fn another_function(x: i32) {
    println!("Another function passing {x}");
}

fn print_labeled_measurements(measurement: u32, units: char) {
    let return_value = {
        let somevar = measurement;
        somevar + 1
    };
    println!("Return value is {return_value}");

    println!("It measures {measurement} {units}s");
    let fivev  = five();
    println!("Fivev is {fivev}");
    let six = plus_one(five());
    println!("Six is {six}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
