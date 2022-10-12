use smart_add_one_new;

fn main() {
    let num = 10;
    print(num);
}

fn print(num: i32) {
    println!(
        "Hello, world! {num} plus one is {}!",
        smart_add_one_new::add_one(num)
    );

    println!("additional info");
    println!("refactor-smart-adder branch");
}
