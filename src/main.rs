use std::convert::TryInto;
use std::convert::TryFrom;

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

        None
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

fn main() {
    //println!("Hello, world!");
    //generate_numbers()
    let mut solutions = [false; 900];
    step(&mut solutions, [1, 3, 5, 25, 50, 100]);
    for (i, solution) in solutions.iter().enumerate() {
        if !*solution {
            println!("{}", i+100)
        }
    }
}

// log2(100*75*50*25*10*10) = 30
// u32::MAX means empty as is shouldn't be reachable
fn step(solutions: &mut [bool; 900], numbers: [u32; 6]) {
    for i in 0..numbers.len() {
        if numbers[i] == u32::MAX { continue };
        for j in i+1..numbers.len() {
            if numbers[j] == u32::MAX { continue; }
            
            {
                let result = numbers[i] + numbers[j];
            
                let mut new_numbers = numbers;
                new_numbers[i] = result;
                new_numbers[j] = u32::MAX;
                
                if (100..1000).contains(&result) {
                    solutions[usize::try_from(result-100).unwrap()] = true;
                }
                
                step(solutions, new_numbers);
            }
            
            {
                let result = numbers[i] * numbers[j];
                
                let mut new_numbers = numbers;
                new_numbers[i] = result;
                new_numbers[j] = u32::MAX;
                
                if (100..1000).contains(&result) {
                    solutions[usize::try_from(result-100).unwrap()] = true;
                }
                
                step(solutions, new_numbers);
            }
            
            {
                let result = numbers[i] - numbers[j];
                if result > 0 {
                    let mut new_numbers = numbers;
                    new_numbers[i] = result;
                    new_numbers[j] = u32::MAX;
                    
                    if (100..1000).contains(&result) {
                        solutions[usize::try_from(result-100).unwrap()] = true;
                    }
                    
                    step(solutions, new_numbers);
                }
            }
            
            {
                let result = numbers[j] - numbers[i];
                if result > 0 {
                    let mut new_numbers = numbers;
                    new_numbers[i] = result;
                    new_numbers[j] = u32::MAX;
                    
                    if (100..1000).contains(&result) {
                        solutions[usize::try_from(result-100).unwrap()] = true;
                    }
                    
                    step(solutions, new_numbers);
                }
            }
            
            {
                if numbers[j] != 0 && numbers[i] % numbers[j] == 0 {
                    let result = numbers[i] / numbers[j];
                    
                    let mut new_numbers = numbers;
                    new_numbers[i] = result;
                    new_numbers[j] = u32::MAX;
                    
                    if (100..1000).contains(&result) {
                        solutions[usize::try_from(result-100).unwrap()] = true;
                    }
                    
                    step(solutions, new_numbers);
                }
            }
            
            {
                if numbers[i] != 0 && numbers[j] % numbers[i] == 0 {
                    let result = numbers[j] / numbers[i];
    
                    let mut new_numbers = numbers;
                    new_numbers[i] = result;
                    new_numbers[j] = u32::MAX;
                    
                    if (100..1000).contains(&result) {
                        solutions[usize::try_from(result-100).unwrap()] = true;
                    }
                    
                    step(solutions, new_numbers);
                }
            }
        }
    }
}



