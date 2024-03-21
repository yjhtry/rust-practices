macro_rules! my_vec {
    () => {
        std::vec::Vec::new()
    };
    ($($el:expr),*) => ({
        let mut v = std::vec::Vec::new();
        $(v.push($el);)*

        v
    });
    ($el:expr; $n:expr) => {
        std::vec::from_elem($el, $n)
    }
}

fn main() {
    let empty: Vec<()> = my_vec!();

    println!("{:?}", empty);

    let nums = my_vec!(1, 2, 3);

    println!("{:?}", nums);

    let nums2 = my_vec!(1; 10);

    println!("{:?}", nums2);
}
