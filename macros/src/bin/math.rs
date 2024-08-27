use macros::add;

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

fn main() {
    run_add();
}
