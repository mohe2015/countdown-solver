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
        // 2-5 numbers needed (maybe specialize for every one of these)

        // more stupid and less efficident but easier implementation
        // TODO FIXME this creates lots of duplicates as the bits at the end that aren't used are changed only
        loop {
            if self.permutation >= (1 << 26) {
                return None;
            }

            let mut index = 0;
            let mut add: u32 = 4;
            for i in 2..22 {
                if ((self.permutation >> (23-i)) & 1) == 1 {
                    result[index] = i/2;
                    index += 1;
                }
                if index == length { // fully filled free places
                    add = 0;
                    self.permutation = (self.permutation >> (23-i)) << (23-i);
                    self.permutation += 1 << (24-i);
                    break;
                }
            }
            self.permutation += add; // lowest bits not used in loop
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
    let mut counter: u64 = 0;
    for numbers in countdown_numbers {
        counter = counter + u64::from(numbers[0]); 
        //println!("{:?}", numbers);
    }
    println!("{}", counter);
}

struct OperatorCombinationIterator {
    combination: u32
}

impl Iterator for OperatorCombinationIterator {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        // maybe do it the over way around? (bit representation)
        // lowest 16 bits are lowest branches
        // then 8, 4, 2, and 1 is the top

        // shift right:
        // 31 [30] [29 28] [27 26 25 24] [23 22 21 20 19 18 17 16] [15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0]
        // count leaves:
        // 30 && !29 + 30 && !28
        // 29 & !27 + 29 & !26
        // 15 + ... + 0

        // top down should be more efficient?
        
        'outer: loop {
            if self.combination & (1 << 31) != 0 {
                return None;
            }

            let mut valid: u32 = 1;
            let mut leaves: u32 = 0;
            
            let combination_complement = !self.combination;
            for i in 0..30 {
                leaves += ((combination_complement >> i) & (self.combination >> (i/2+16))) & 1;
                valid &= (combination_complement >> i) | (self.combination >> (i/2+16));
                if (valid & 1) == 0 || leaves > 6 {
                    self.combination += 1;
                    continue 'outer;
                }
            }

            self.combination += 1;
            break;
        }

        Some(self.combination)
    }
}

fn generate_operator_combinations() {
    let operator_combinations = OperatorCombinationIterator {
        combination: 0
    };
    let mut counter: u64 = 0;
    for combination in operator_combinations {
        counter += u64::from(combination); 
        //println!("{}", combination);
    }
    println!("{}", counter);
}

fn generate_operator_orders() {
    // six numbers
    // allowed ops: + - * /
    // order choosen freely
    // not all numbers need to be used but none can be used twice
    // division only without remainder
    // only positive numbers at any stage
    // + and * is commutative
    // / and - can only be used rarely properly (underflow, remainder)
    
    // starting with
    // a + b
    // a - b
    // a * b
    // a / b
    // b - a
    // b / a

    // YES: maybe fixed number parameter order? maybe also generate b + c etc.?
    // probably something recursive? (write comparison function?)
    // in theory a valid a - b would exclude b - a but I think this is too complicated

    // maybe sort them so its easy to detect commutativity
    // maybe memoization helps
}

fn main() {
    //println!("Hello, world!");
    //generate_numbers()
    generate_operator_combinations()
}
