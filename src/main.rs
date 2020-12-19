use std::convert::TryInto;

struct CountdownNumberIterator {
    permutation: u32
}

impl Iterator for CountdownNumberIterator {
    type Item = [u8; 6];

    fn next(&mut self) -> Option<[u8; 6]> {
        // this array would be about 1MB of storage

        // only sort ascending which prevents duplicates

        // TODO CHECK
        const MAJOR_NUMBERS: [[u8; 6]; 16] = [
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

        // TODO CHECK
        const LENGTH: [usize; 16] = [5,5,5,5,4,4,4,4,4,4,3,3,3,3,3,2];

        let big_number_permutation: usize = ((self.permutation >> 22) & 0b1111).try_into().unwrap(); // TODO FIXME I don't understand why I need this
        let mut result = MAJOR_NUMBERS[big_number_permutation]; // TODO FIXME does this break the original array contentes?
        let length = LENGTH[big_number_permutation];

        /*
        // this would be the more efficient solution but it has problems
        let mut index = 0;
        for i in (0..20).step_by(2) {
            let state = (self.permutation >> i) & 0b11;
            if state == 2 {
                result[index] = i/2+1;
                index += 1;
                // TODO FIXME this could overflow
                result[index] = i/2+1;
                index += 1;
            }
            if state == 1 {
                result[index] = i/2+1;
                index += 1;
            }
        }
        // 2 bits per small number (one invalid state)
        // for every number from 1 - 10 we can choose (), (x), (x x) - reverse doesnt matter
        // so choose one of these until you got the amount of numbers needed
        */
       
        // small numbers
        // 2-5 numbers needed

        // more stupid and less efficident but easier implementation
        // TODO FIXME this creates lots of duplicates as the bits at the end that aren't used are changed only
        loop {
            let mut index = 0;
            for i in 2..22 {
                if ((self.permutation >> i) & 1) == 1 {
                    result[index] = i/2;
                    index += 1;
                }
                if index == length {
                    break;
                }
            }
            self.permutation += 4; // lowest bits not used in loop
            if index == length { // otherwise not enough numbers where used (0s left)
                break;
            }
        }
        
        Some(result)
    }
}

fn generate_numbers() {
    let countdown_numbers = CountdownNumberIterator {
        permutation: 0
    };
    for numbers in countdown_numbers {
        println!("{:?}", numbers);
    }
}

fn main() {
    println!("Hello, world!");
    generate_numbers()
}
