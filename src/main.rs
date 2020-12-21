use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;

// (25+2)*3*2+1+1

fn generate_small_numbers(mut numbers: [i32; 6], index: usize, max_number: i32) {
    for x in (2..max_number + 1).rev() {
        if x % 2 == 0 && (index == 0 || numbers[index - 1] != x / 2) {
            continue;
        }

        // 1 = 2, 3
        // 10 = 20, 21

        numbers[index] = x / 2;

        if index == 5 {
            print!("{{\"key\": {:?}, \"value\": [", numbers);

            let solutions = step(numbers);

            let mut first = true;
            for i in 0..900 {
                if ((solutions[i / 8] >> (i % 8)) & 1) == 0 {
                    if first {
                        print!("{}", i + 100);
                        first = false;
                    } else {
                        print!(", {}", i + 100);
                    }
                }
            }
            // 25, 50, 75, 100, 1, 1 is the last one
            if numbers[5] == 1 && numbers[4] == 1 && numbers[3] == 100 && numbers[2] == 75 && numbers[1] == 50 && numbers[0] == 25 {
                println!("]}}");
            } else {
                println!("]}},");
            }
        } else {
            generate_small_numbers(numbers, index + 1, x - 1);
        }
    }
}
/*
fn generate_big_numbers(numbers: [i32; 6], min_removed_number: i32) {


    // maybe always remove one of them?
    // only remove smaller numbers in subsequent invocations?
}
*/
fn generate_numbers() {
    // TODO CHECK
    const MAJOR_NUMBERS: [[i32; 6]; 16] = [
        [0, 0, 0, 0, 0, 0],
        [25, 0, 0, 0, 0, 0],
        [50, 0, 0, 0, 0, 0],
        [75, 0, 0, 0, 0, 0],
        [100, 0, 0, 0, 0, 0],
        [25, 50, 0, 0, 0, 0],
        [25, 75, 0, 0, 0, 0],
        [25, 100, 0, 0, 0, 0],
        [50, 75, 0, 0, 0, 0],
        [50, 100, 0, 0, 0, 0],
        [75, 100, 0, 0, 0, 0],
        [25, 50, 75, 0, 0, 0],
        [25, 50, 100, 0, 0, 0],
        [25, 75, 100, 0, 0, 0],
        [50, 75, 100, 0, 0, 0],
        [25, 50, 75, 100, 0, 0],
    ];

    // TODO CHECK
    const INDEX: [usize; 16] = [0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 4];

    for i in 0..MAJOR_NUMBERS.len() {
        generate_small_numbers(MAJOR_NUMBERS[i], INDEX[i], 21);
    }
}

thread_local! {
    pub static MEMOIZATION: RefCell<HashMap<[i32; 6], [u8; 128]>> = RefCell::new(HashMap::new());
}

fn main() {
    println!("[");
    generate_numbers();
    println!("]");

    // MEMOIZATION.with(|m| {
    //     for (key, value) in &(*m.borrow()) {
    //        eprintln!("{:?}: {:?}", key, value);
    //    }
    //});
}

// log2(100*75*50*25*10*10) = 30
// u32::MAX means empty as is shouldn't be reachable

enum Operation {
    ADDITION,
    MULTIPLICATION,
    SUBTRACTION,
    ReverseSubtraction,
    DIVISION,
    ReverseDivision,
}

fn step(numbers: [i32; 6]) -> [u8; 128] {
    let mut amount_of_numbers = 0;
    for (i, number) in numbers.iter().enumerate() {
        if *number == i32::MAX {
            amount_of_numbers = i + 1;
            break;
        }
    }
    
    //println!("{:?}", numbers);
    if amount_of_numbers > 3 {
        let is_cached = MEMOIZATION.with(|m| {
            if let Some(cached_solutions) = m.borrow_mut().get(&numbers) {
                Some(*cached_solutions)
            } else {
                None
            }
        });
        if let Some(solutions) = is_cached {
            return solutions;
        }
    }

    let mut solutions = [0; 128];

    for i in 0..numbers.len() {
        if numbers[i] == i32::MAX {
            break;
        }

        if (100..1000).contains(&numbers[i]) {
            let index = usize::try_from(numbers[i] - 100).unwrap();
            solutions[index / 8] |= 1 << (index % 8);
        }

        for j in i + 1..numbers.len() {
            if numbers[j] == i32::MAX {
                break;
            }

            for operation in &[
                Operation::ADDITION,
                Operation::MULTIPLICATION,
                Operation::SUBTRACTION,
                Operation::ReverseSubtraction,
                Operation::DIVISION,
                Operation::ReverseDivision,
            ] {
                let result = match operation {
                    Operation::ADDITION => numbers[i] + numbers[j],
                    Operation::MULTIPLICATION => numbers[i] * numbers[j],
                    Operation::SUBTRACTION => {
                        let result = numbers[i] - numbers[j];
                        if result < 0 {
                            continue;
                        }
                        result
                    }
                    Operation::ReverseSubtraction => {
                        let result = numbers[j] - numbers[i];
                        if result < 0 {
                            continue;
                        }
                        result
                    }
                    Operation::DIVISION => {
                        if numbers[j] != 0 && numbers[i] % numbers[j] == 0 {
                            numbers[i] / numbers[j]
                        } else {
                            continue;
                        }
                    }
                    Operation::ReverseDivision => {
                        if numbers[i] != 0 && numbers[j] % numbers[i] == 0 {
                            numbers[j] / numbers[i]
                        } else {
                            continue;
                        }
                    }
                };
                
                // optimization
                if amount_of_numbers == 2 {
                    if (100..1000).contains(&result) {
                        let index = usize::try_from(result - 100).unwrap();
                        solutions[index / 8] |= 1 << (index % 8);
                    }
                } else {
                    let mut new_numbers = numbers;
                    new_numbers[i] = result;
                    for k in j..new_numbers.len() - 1 {
                        new_numbers[k] = new_numbers[k + 1];
                    }
                    new_numbers[new_numbers.len() - 1] = i32::MAX;
    
                    let inner_solutions = step(new_numbers);
                    for k in 0..solutions.len() {
                        solutions[k] |= inner_solutions[k];
                    }
                }
            }
        }
    }

    if amount_of_numbers > 3 {
        MEMOIZATION.with(|m| {
            m.borrow_mut().insert(numbers, solutions);
        });
    }

    solutions
}
