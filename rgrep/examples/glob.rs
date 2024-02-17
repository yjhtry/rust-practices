use glob::glob;

fn main() {
    let entries = glob("src/**/*.rs").expect("Failed to read glob pattern");

    for entry in entries {
        match entry {
            Ok(path) => {
                let content = std::fs::read_to_string(path).expect("Failed to read file");

                println!("{}", content);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
