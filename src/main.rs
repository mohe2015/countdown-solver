struct CountdownNumberIterator {
    index: u64
}

impl Iterator for CountdownNumberIterator {
    type Item = [u8; 6];

    fn next(&mut self) -> Option<[u8; 6]> {
        // this array would be about 1MB of storage

        // only sort ascending which prevents duplicates

        // TODO CHECK
        let major_numbers: [[u8; 6]; 16] = [
            [0, 0, 0,  0,  0,  25 ],
            [0, 0, 0,  0,  0,  50 ],
            [0, 0, 0,  0,  0,  75 ],
            [0, 0, 0,  0,  0,  100],
            [0, 0, 0,  0,  25, 50 ],
            [0, 0, 0,  0,  25, 75 ],
            [0, 0, 0,  0,  25, 100],
            [0, 0, 0,  0,  50, 75 ],
            [0, 0, 0,  0,  50, 100],
            [0, 0, 0,  0,  75, 100],
            [0, 0, 0,  25, 50, 75 ],
            [0, 0, 0,  25, 50, 75 ],
            [0, 0, 0,  25, 50, 100],
            [0, 0, 0,  25, 75, 100],
            [0, 0, 0,  50, 75, 100],
            [0, 0, 25, 50, 75, 100]
        ];

        // 2 bits per small number (one invalid state)

        // small numbers
        // 2-5 numbers needed

        // for every number from 1 - 10 we can choose (), (x), (x x) - reverse doesnt matter
        // so choose one of these until you got the amount of numbers needed

        Some(major_numbers[0])
    }
}

fn generate_numbers() {}

fn main() {
    println!("Hello, world!");
}
