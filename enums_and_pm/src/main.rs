// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6
// }
// enum IpAddr {
//     V4(String),
//     V6(String)
// }

// impl IpAddr {
//     // fn display(self: &IpAddr) {
//     //     // println!("Kind is {:?}", self.kind);
//     //     // println!("Address is {}", self.address);
//     // }
// }

// #[derive(Debug)]
// enum Option<T> {
//     None,
//     Some(T)
// }



// fn divide_by_two(a: i32) -> Option::<i32> {
//     if (a % 2) != 0 {
//         let b : Option<i32> = None;
//         b
//     }
//     Option::<i32>::Some(a / 2)
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("MaximUM");
            25
        }
    }
}


fn main() {
    let coin_a = Coin::Dime;
    let coin_b = Coin::Quarter;
    println!("{}", value_in_cents(coin_a) + value_in_cents(coin_b));
    let a = Some(9);
    //    println!("{:?}", divide_by_two(a));
    let b = Some('e');
    println!("{:?}", b);
    println!("{:?}", a);
    let nothing : Option<i32> = None;
    println!("{:?}", nothing);
    //    println!("{:?}", divide_by_two(b));
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // let my_local_ip = IpAddr::V4(String::from("127.0.0.1"));
    // let another_ip = IpAddr::V6(String::from("127.0.0.1"));
    // // println!("{}", four);
    // // dbg!("{}", six);
    // // route(IpAddrKind::V4);
    // // let my_local_ip = IpAddr {
    // //     kind: IpAddrKind::V4,
    // //     address: String::from("192.168.1.55")
    // // };
    // //    my_local_ip.display();

    // #[derive(Debug)]
    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32)
    // }
    // impl Message {
    //     fn call(&self) {
    //         if self::Write(message)
    //         let message =
    //         println!("{}", Message::Write());
    //     }
    // }
    // let m = Message::Write(String::from("Hola cómo va?"));
    // m.call();
}

// fn route(ip_kind: IpAddrKind) {
//     dbg!(ip_kind);
// }
