trait Dump {
    fn dump(&self);
}

#[derive(bar_macro::Dump)]
struct Baz {
    x: u8,
    y: u32,
}

fn main() {
    Baz { x: 10, y: 42 }.dump();
    // こんな出力
    // struct Baz {
    //    x: 10,
    //    y: 42,
    // }
}
