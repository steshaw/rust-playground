use std::fmt;
use std::fmt::Formatter;
// Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: i8,
    y: i8,
}

// Similarly, implement `Display` for `Point2D`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x: ")?;
        fmt::Binary::fmt(&self.x, f)?;
        write!(f, ", y: ")?;
        fmt::Binary::fmt(&self.y, f)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("MinMax:");
    println!(" Display: {}", minmax);
    println!(" Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big_range} and the small is {small_range}");

    let point = Point2D { x: 3, y: 7 };

    println!("Point2D:");
    println!("  Display: {}", point);
    println!("  Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // Update: We implemented `fmt::Binary` to see what would happen!
    println!("Point2D (as binary):");
    println!("  Display binary: {:b}", point);
    println!("  Debug: {:#b}", point);
}
