// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)


fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
    let mut s = String::from("hello");
    let r1 = &mut s;
    println!("{}", r1);
    r1.push_str(", for real!");
    println!("{}", r1);
    println!("{}", s);
}

fn current_favorite_color() -> String {
    String::from("blue")//.to_string() // ...also works
}
