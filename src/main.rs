struct BigNumberIterator {}

impl Iterator for BigNumberIterator {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        // only sort ascending which prevents duplicates

        // (hopefully all big number possibilities)

        // 25
        // 50
        // 75
        // 100

        // 25, 50
        // 25, 75
        // 25, 100

        // 50, 75
        // 50, 100

        // 75, 100

        // 25, 50, 75
        // 25, 50, 100
        // 25, 75, 100

        // 50, 75, 100

        // 25, 50, 75, 100

        Some(1)
    }
}

fn generate_numbers() {}

fn main() {
    println!("Hello, world!");
}
