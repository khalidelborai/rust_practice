use macros::{add, add_as};

fn run_add() {
    let x = 1;
    let y = 2;
    let z = add!(x, y);
    let f = add!(x, y, z);
    let single = add!(x);
    println!("{} = {}", x, single);
    println!("{} + {} = {}", x, y, z);
    println!("{} + {} + {} = {}", x, y, z, f);
}

fn run_add_as() {
    let x = 1;
    let y = 2;
    let z = add_as!(x, y, i32);
    let f = add_as!(x, y, z, i32);
    let single = add_as!(x, i32);
    println!("{} = {}", x, single);
    println!("{} + {} = {}", x, y, z);
    println!("{} + {} + {} = {}", x, y, z, f);
}

fn main() {
    run_add();
    run_add_as();
}
