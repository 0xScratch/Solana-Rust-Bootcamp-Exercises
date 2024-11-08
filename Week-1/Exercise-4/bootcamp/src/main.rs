fn fizz_buzz_1() {
    let mut n_fizz_buzz = 0;

    for num in 0..=301 {
        match (num % 3, num % 5) {
            (0, 0) => {
                println!("fizz buzz");
                n_fizz_buzz += 1;
            },
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => continue
        }
    }

    println!("'fizz buzz' occurred {} times\n", n_fizz_buzz);
}

fn fizz_buzz_2() {
    let mut n_fizz_buzz = 0;

    for num in 0..=301 {
        if num % 3 == 0 && num % 5 == 0 {
            println!("fizz buzz");
            n_fizz_buzz += 1;
        }
        else if num % 3 == 0 {
            println!("fizz");
        }
        else if num % 5 == 0 {
            println!("buzz");
        }
    }

    println!("'fizz buzz' occurred {} times", n_fizz_buzz);
}

fn main() {
    println!("Welcome to the Rust World!");

    // Fizz Buzz using `match` expression
    fizz_buzz_1();

    // Fizz Buzz using `if else` condition
    fizz_buzz_2();
}
