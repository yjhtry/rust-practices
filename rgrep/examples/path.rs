fn main() {
    let pwd = std::env::current_dir().unwrap();
    let path = pwd.join("src/main.rs");

    println!("{:?}", path);
}
