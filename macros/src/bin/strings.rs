use macros::{repeat, trim_and_split, trim_and_split_on};

fn run_repeat() {
    let s = "Hello, World!";
    let r = repeat!(3, s);
    for i in r {
        println!("{}", i);
    }
}

fn run_trim_and_split() {
    let s = " Hello, World! ";
    let r = trim_and_split!(s, ",");
    println!("{:?}", r);
}

fn run_trim_and_split_on() {
    let s = "-Hello, World!-";
    let r = trim_and_split_on!(s, '-', ",");
    println!("{:?}", r);
}

fn main() {
    run_repeat();
    run_trim_and_split();
    run_trim_and_split_on();
}
