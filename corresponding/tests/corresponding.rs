use corresponding::*;

#[derive_corresponding]
mod move_corresponding {
    #[derive(Default, Debug, Clone, Eq, PartialEq)]
    pub struct A {
        // Same vs other type
        pub a: u8,
        pub b: u8,

        // Option<T> to T
        pub c: u8,
        pub d: u8,

        // Option<T> to Option<T>
        pub e: Option<u8>,
        pub f: Option<u8>,
        pub g: Option<u8>,
        pub h: Option<u8>,

        // T to Option<T>
        pub i: Option<u8>,
        pub j: Option<u8>,

        // Heap values
        pub k: String,
        pub l: Box<u8>,

        // Only in A
        pub m: u8,
    }

    pub struct B {
        // Same vs other type
        pub a: u8,
        pub b: u16,

        // Option<T> to T
        pub c: Option<u8>,
        pub d: Option<u8>,

        // Option<T> to Option<T>
        pub e: Option<u8>,
        pub f: Option<u8>,
        pub g: Option<u8>,
        pub h: Option<u8>,

        // T to Option<T>
        pub i: u8,
        pub j: u8,

        // Heap values
        pub k: String,
        pub l: Box<u8>,

        // Only in B
        pub n: u8,
    }
}

pub use move_corresponding::*;

#[test]
fn test_move_corresponding() {
    let mut a = A {
        a: 1,
        b: 1,
        c: 1,
        d: 1,
        e: None,
        f: None,
        g: Some(1),
        h: Some(1),
        i: None,
        j: Some(1),
        k: "1".to_string(),
        l: Box::new(1),
        m: 1,
    };

    let b = B {
        a: 2,
        b: 2,
        c: None,
        d: Some(2),
        e: None,
        f: Some(2),
        g: None,
        h: Some(2),
        i: 2,
        j: 2,
        k: "2".to_string(),
        l: Box::new(2),
        n: 2,
    };

    a.move_corresponding(b);

    assert_eq!(
        a,
        A {
            a: 2,
            b: 1,
            c: 1,
            d: 2,
            e: None,
            f: Some(2),
            g: Some(1),
            h: Some(2),
            i: Some(2),
            j: Some(2),
            k: "2".to_string(),
            l: Box::new(2),
            m: 1,
        }
    );
}

#[test]
fn test_into() {
    let a: A = B {
        a: 2,
        b: 2,
        c: None,
        d: Some(2),
        e: None,
        f: Some(2),
        g: None,
        h: Some(2),
        i: 2,
        j: 2,
        k: "2".to_string(),
        l: Box::new(2),
        n: 2,
    }
    .into();

    assert_eq!(
        a,
        A {
            a: 2,
            b: 0,
            c: 0,
            d: 2,
            e: None,
            f: Some(2),
            g: None,
            h: Some(2),
            i: Some(2),
            j: Some(2),
            k: "2".to_string(),
            l: Box::new(2),
            m: 0,
        }
    );
}
