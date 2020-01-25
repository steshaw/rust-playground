// Inspired by https://stackoverflow.com/a/58018832/482382

#[derive(Debug, PartialEq)]
struct A<'a>(&'a str);

#[derive(Debug, PartialEq)]
struct B<'a, 'b> {
    a: &'a A<'b>,
    msg: &'a str,
}

//fn foo() {}

fn main() {
    let s = "a".to_string();
    let a = A(&s);
    assert_eq!(a, A("a"));
    let b = B { a: &a, msg: "Hi" };
    assert_eq!(
        b,
        B {
            a: &A("a"),
            msg: "Hi"
        }
    );
    let s1 = b.a.0;
    assert_eq!(s1, s);

    let c = "c".to_string();
    let r = B {
        a: &A(&c),
        msg: "hi",
    }
    .a
    .0;
    assert_eq!(r, "c");
}
