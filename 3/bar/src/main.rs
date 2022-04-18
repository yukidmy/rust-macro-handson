#[bar_macro::greet2]
fn some() {}

#[bar_macro::trace("Here")]
fn add(x: u32, y: u32) -> u32 {
    x + y
}

#[bar_macro::trace("There")]
fn add_sub(x: u8, y: u8, z: u8) -> u8 {
    x + y - z
}

fn main() {
    greet2();
    assert_eq!(add(1, 2), 3); // "add -> 3" とprint
    assert_eq!(add_sub(3, 4, 5), 2); // "add_sub -> 2"とprint
}
