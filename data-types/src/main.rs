fn for_loop_main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is : {element}")
    }

    for number in (1..=10).rev() {
        println!("{number}")
    }
    println!("hello")
}

fn loop_example_two() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            };
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}")
}

fn loop_example() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("{}", counter);
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn labeled_ms(value: i32, label: char) {
    println!("hello labeled_ms {value} {label}")
}

fn another_fn(x: i32) {
    println!("hello another_fn {x}")
}

fn main() {
    another_fn(90);
    labeled_ms(90, 's');

    let x = five();
    println!("{x}");
    println!("{}", plus_one(x));

    loop_example();
    loop_example_two();
    for_loop_main();
}
