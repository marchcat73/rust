fn double(n: i32) -> i32 {
    n * 2
}

// fn double_or_noting(n: i32) -> i32 {
//     if n > 0 {
//         n * 2
//     } else {
//         0
//     }
// }

fn greet(s: String) -> String {
    println!("Hello {s}");
    s
}

fn greet_borrow(s: &String) {
    println!("{s}");
}

fn greet_borrow_mut(s: &mut String) {
    *s = format!("greet_borrowMut {s}");
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input
}

fn main() {
    // let mut n: i32 = 5;
    // n += 1;

    // {
    //     let n: i32 = 5;
    //     let n: i32 = n + 1;

    //     println!("{n}");
    // }

    // println!("{n}");

    // let m: i32 = double(5);
    // println!("{}", m);

    // let mut name = "Hello".to_string();

    // greet(name.clone());
    // // greet(name);
    // greet_borrow(&name);

    // greet_borrow_mut(&mut name);
    // println!("{}", name);

    let input = read_line();
    println!("You typed: [{input}]");
}
