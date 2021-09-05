fn main() {
    let something_to_pass = "HERE YOU GO!";
    let thing_to_return = "here we are";

    cool_func(something_to_pass);
    let new_thing_returned = other_func(thing_to_return);
    println!("new entity, returned: {}", new_thing_returned);
}

fn cool_func(something: &str) {
    println!("Here is your something:");
    println!("{}, cool right?", something);
}

fn other_func<T>(returnable: T) -> T {
    println!("here is your thing back");
    returnable
}
