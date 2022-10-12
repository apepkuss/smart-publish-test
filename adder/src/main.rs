use smart_add_one_new;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {num} plus one is {}!",
        smart_add_one_new::add_one(num)
    );

    println!("additional info");
}
