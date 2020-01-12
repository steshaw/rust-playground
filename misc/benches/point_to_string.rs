#![feature(test)]

extern crate test;

use test::Bencher;

struct Point {
    x: f64,
    y: f64,
}

const P: Point = Point { x: 2.3, y: 12.4 };

/*
#[bench]
fn array_concat(b: &mut Bencher) {
    b.iter(|| {
        r: &str = &[DATE, T, TIME].concat();
        test::black_box(datetime);
    });
}
*/

#[bench]
fn array_join_long(b: &mut Bencher) {
    b.iter(|| {
        let r: &str = &[
            "(",
            P.x.to_string().as_str(),
            ", ",
            P.y.to_string().as_str(),
            ")",
        ]
        .join("");
        test::black_box(r);
    });
}

#[bench]
fn format(b: &mut Bencher) {
    b.iter(|| {
        let r = format!("({}, {})", P.x, P.y);
        test::black_box(r);
    });
}

#[bench]
fn push_str(b: &mut Bencher) {
    b.iter(|| {
        let mut r = String::from("(");
        r.push_str(P.x.to_string().as_str());
        r.push_str(", ");
        r.push_str(P.y.to_string().as_str());
        r.push_str(")");
        test::black_box(r);
    });
}

#[bench]
fn push_str_with_capacity(b: &mut Bencher) {
    b.iter(|| {
        let mut r = String::with_capacity(30);
        r.push_str("(");
        r.push_str(P.x.to_string().as_str());
        r.push_str(", ");
        r.push_str(P.y.to_string().as_str());
        r.push_str(")");
        test::black_box(r);
    });
}
