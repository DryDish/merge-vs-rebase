fn main() {
    let something_to_pass = "HERE YOU GO!";
    cool_func(something_to_pass);
}

fn cool_func(something: &str) {
    println!("Here is your something:");
    println!("{}, cool right?", something);
}
