use smart_add_one_new;

fn main() {
    let num = 10;
    plus_one(num);
}

fn plus_one(num: i32) {
    let plus_one = smart_add_one_new::add_one(num);
    println!("Hello, world! {num} plus one is {}!", echo_num(plus_one),);

    println!("additional info");
    println!("refactor-smart-adder branch");
}

fn echo_num(num: i32) -> i32 {
    num
}
