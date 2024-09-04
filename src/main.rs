#[derive(Debug)]
struct Box {
    name: String,
    width: u32,
    length: u32,
    height: u32
}

impl Box {
    fn new(name: String, width: u32, length: u32, height: u32) -> Self {
        Self {
            name,
            width,
            length,
            height
        }
    }
}

fn main() {
    println!("Welcome to BoxMaker!");
    let test_box: Box = Box::new(String::from("Test Box"), 10, 10, 10);
    println!("test_box: {test_box:?}");
}
