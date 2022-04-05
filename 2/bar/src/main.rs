fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

bar_macro::greet!(hello);

bar_macro::printable! { enum E { A, B, C, } }

bar_macro::make_fieldless! { enum Param {
    A(u8),
    B { x: f32, y: f64 },
    C,
} }

fn main() {
    greet();

    E::A.print(); // E::A

    print_type_of(&Param::A(5).convert());
}
