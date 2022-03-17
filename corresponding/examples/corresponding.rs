use corresponding::*;

#[derive_corresponding]
mod mc {
    #[derive(Default, Debug, Clone, Copy)]
    pub struct X {
        pub a: u8,
        pub b: u8,
    }

    #[derive(Default, Debug, Clone)]
    pub struct Y {
        pub a: u8,
        pub b: u16,
        pub c: u8,
    }
}

pub use mc::*;

fn main() {
    let mut x = X { a: 1, b: 2 };
    let mut y = Y { a: 3, b: 5, c: 4 };
    let mut z = X::default();

    x.clone_corresponding(&y);
    z.copy_corresponding(&x);
    y.move_corresponding(X::default());
    
    println!("{x:#?}");
    println!("{y:#?}");
    println!("{z:#?}");
}