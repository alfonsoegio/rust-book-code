use std::fs::{File, self};
use std::io::ErrorKind;
use std::io::{self, Read};
use std::net::IpAddr;

fn main() {
    println!("Hello, world!");
    // panic!("crash and burn!");
    // let v = vec![1, 2, 3];
    // v[99];
    let greeting_file_result = File::open("hello.txt");
    println!("{:?}", greeting_file_result);
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problems creating file {:?}", e)
            },
            other_error => {
                panic!("Don't know what to do {:?}", other_error);
            }
        }
    };
    println!("{:?}", greeting_file);

    let another_file = File::open("goodbye.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("goodbye.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });
    println!("{:?}", another_file);

    // let third_file = File::open("abc.txt").unwrap();
    // println!("{:?}", third_file);

    let third_file = File::open("abc.txt")
        .expect("abc.txt Should be included in this project");
    println!("{:?}", third_file);
    let username_read_result = read_username_from_file();
    let name = match username_read_result {
        Ok(name) => {
            name
        },
        Err(err) => {
            println!("{:?}", err);
            format!("{:?}", err)
        }
    };
    println!("{}", name);
    let name_2 = read_username_from_file_2();
    match name_2 {
        Ok(name) => println!("name is {:?}", name),
        Err(err) => println!("ErRoR => {:?}", err)
    };
    let name_3 = read_username_from_file_3();
    match name_3{
        Ok(name) => println!("name 3 is {:?}", name),
        Err(err) => println!("ErRoR 3 => {:?}", err)
    };

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("{:?}", home);
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => return Ok(username),
        Err(e) => return Err(e)
    }
}
