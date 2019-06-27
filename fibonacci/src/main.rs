use std::io;

fn main() {
    loop {
        println!("Get fibonacci of:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(err) => {
                println!("{:?}", err);
                continue;
            },
        }

        let input: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{:?}", err);
                continue;
            },
        };

        println!("{:?}", fib(input));
        break;
    }
}

fn fib(x: u64) -> u64 {
    match x {
        0 => 0,
        1 | 2 => 1,
        _ => fib(x - 1) + fib(x - 2)
    }
}
