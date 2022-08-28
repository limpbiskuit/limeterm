fn main() {
    let a = std::env::var("SHELL").unwrap();

    println!("{}", a);
}