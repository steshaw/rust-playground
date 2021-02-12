fn main() {
    let len = {
        let s1 = String::from("123");
        let mut s2 = s1;
        s2.push_str("456");
        // s1 is inaccessible here.
        /* s1.len() +  */ s2.len()
    };
    println!("len = {}", len)
}
