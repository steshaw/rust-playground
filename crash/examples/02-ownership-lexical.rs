#[derive(Debug)]
struct Var(String);

impl Drop for Var {
    fn drop(&mut self) {
        println!("Dropping a Var: {:?}", self);
    }
}

fn main() {
    println!("Before x");
    let _x = Var("_x".to_string());
    println!("After x");
//    {
        println!("Before y");
        let _y = Var("_y".to_string());
        println!("After y");
 //   }
    println!("End of main");
}
