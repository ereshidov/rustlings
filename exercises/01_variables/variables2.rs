// reason => x never was initialized, Rust requires
// variables to have a defined value before use
fn main() {
    let x: i32 = 10;

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
