enum Test {
    A,
}

// Recursive call to fmt()
impl core::fmt::Debug for Test {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

fn main() {
    let a = Test::A;
    println!("{:?}", a);
}
