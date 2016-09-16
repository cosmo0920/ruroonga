/// rustc mydrop.rs
/// ./mydrop
struct MyDrop;

impl Drop for MyDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() {
    let _x = MyDrop;
}
// #=> Dropping!
