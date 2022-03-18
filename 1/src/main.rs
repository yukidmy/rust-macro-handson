#[derive(Debug)]
struct Foo;

macro_rules! greet {
    (hello) => {
        println!("Hello world!");
    };
    (goodbye) => {
        println!("Bye bye!");
    };
}

macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

macro_rules! sum {
    ($($x:expr),*) => {{
        let mut s = 0;
        $(s += $x;)*
        s
    }};
}

// Q1
// let h = hash_map!("a" => 1, "b" => 2);
macro_rules! hash_map {
    ($($k:expr => $v:expr),*) => {{
        let mut result = ::std::collections::HashMap::new();
        $(result.insert($k, $v);)*
        result
    }};
}

// Q2
// printable! { enum E { A, B, C, } }
macro_rules! printable {
    (enum $name:ident { $($variant:ident,)* }) => {
        enum $name { $($variant,)* }
        impl $name {
            fn print(&self) {
                match self {
                    $($variant => println!("{}::{}", stringify!($name), stringify!($variant)),)*
                }
            }
        }
    };
}

printable! { enum E { A, B, C, } }

fn main() {
    greet!(goodbye);

    println!("{}", 3 * add!(1, 2));

    println!("{}", 3 * sum!(1, 2));

    let h = hash_map!("a" => 1, "b" => 2);
    println!("{:?}", h); // {"a": 1, "b": 2}
    println!("{}", h["a"]); // 1

    E::A.print(); // E::A
}
