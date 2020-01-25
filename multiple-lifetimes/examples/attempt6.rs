#[derive(Debug)]
struct A<'a> {
    a: &'a str,
    b: &'a str,
}
#[derive(Debug)]
struct Two<'a> {
    a: &'a A<'a>,
    b: &'a A<'a>,
}
fn foo(s: &str) {
    println!("foo: s = {}", s);
}
fn bar(s: &str) {
    println!("foo: s = {}", s);
}
fn baz(_a: (), _b: ()) {}
/*
fn replace<'a>(t: &'a mut Two, a: &'a A) -> &'a A {
    t.a = a;
}
*/
fn main() {
    let a = A {
        a: "hi",
        b: "there",
    };

    foo(a.a);
    bar(a.b);
    baz(foo(a.a), bar(a.b));
    println!("a = {:?}", a);

    {
        let b = A { a: "b.a", b: "b.b" };
        let t = Two { a: &a, b: &b };
        foo(t.a.a);
        bar(t.a.b);
        baz(foo(t.a.a), bar(t.b.a));
        println!("t = {:?}", t);
    }
    println!("a = {:?}", a);
}
