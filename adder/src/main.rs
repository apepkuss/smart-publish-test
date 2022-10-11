use smart_add_one;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {num} plus one is {}!",
        smart_add_one::add_one(num)
    );
}
