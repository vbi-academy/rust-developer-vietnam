fn main() {
    println!("Hello, world!");

    // let input = "u";

    // if let Some(value) = parse_int(input) {

    //     println!("{value}");

    // }
    // else {

    //     eprintln!("Error happens here");
    // }

    // Phân biệt cách sử dụng của iter() và peek()
    // vec, hashmap

    let v = vec![1, 2, 3, 4, 5];
    for item in v.iter() {
        println!("item:{}", item);
    }

    let v2 = vec![1, 2, 3, 4, 5];
    for item in v2.iter().peekable() {
        println!("item:{}", item);
    }

    // sự khác nhau
    //next() ở iter() thì -> giá trị nó sẽ bị consume (khi call next thì
    // thì vị trí tiếp theo nó đã có rùi
    // peek() ở peekable thì -> giá trị của nó sẽ ko consume -> giá trị tiếp
    // theo chỉ xem qua và thực hiện action nếu mình mong muốn

    let numbers = vec![1, 2, 3, 4, 5];
    let mut iter = numbers.iter();
    while let Some(next) = iter.next() {
        if next == &3 {
            println!("Skipping {}", next);
            iter.next();
        } else {
            println!("{}", next);
            iter.next();
        }
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let mut iter = numbers.iter().peekable();

    while let Some(&next) = iter.peek() {
        if next == &3 {
            println!("Skipping {}", next);
            iter.next(); // consumes the peeked element
        } else {
            println!("{}", next);
            iter.next(); // consumes the peeked element
        }
    }
}

// Traits - định nghĩa giao diện (interface) và chia sẻ hành vi giữa các
// loại dữ liệu khác nhau

pub trait People {
    fn height(&self) -> String;
    fn weight(&self) -> String;
}

struct Peter {}

struct Alice {}

impl People for Peter {
    fn height(&self) -> String {
        "1m70".to_string()
    }

    fn weight(&self) -> String {
        "70kg".to_string()
    }
}

impl People for Alice {
    fn height(&self) -> String {
        "1m80".to_string()
    }

    fn weight(&self) -> String {
        "74kg".to_string()
    }
}

// if let Some thường sử dụng cho Option hoặc Result

pub fn parse_int(input: &str) -> Option<u32> {
    let res = input.parse::<u32>().ok();

    res
}
