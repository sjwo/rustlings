// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)


fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO not sure I understand the relationship between for x in fallible and if let...
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
