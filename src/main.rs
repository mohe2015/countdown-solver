use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;

fn generate_small_numbers(mut numbers: [i32; 6], index: usize, max_number: i32) {
    for x in (2..max_number + 1).rev() {
        if x % 2 == 0 && numbers[index - 1] != x / 2 {
            continue;
        }

        // 1 = 2, 3
        // 10 = 20, 21

        numbers[index] = x / 2;

        if index == 5 {
            print!("{{key: {:?}, value: [", numbers);

            let solutions = step(numbers);

            for i in 0..900 {
                if ((solutions[index / 8] >> (i % 8)) & 1) == 0 {
                    print!("{},", i + 100)
                }
            }
            println!("]}},");
        } else {
            generate_small_numbers(numbers, index + 1, x - 1);
        }
    }
}

fn generate_numbers() {
    // TODO CHECK
    const MAJOR_NUMBERS: [[i32; 6]; 15] = [
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
    const INDEX: [usize; 15] = [1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 4];

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

fn step(numbers: [i32; 6]) -> [u8; 128] {
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

    let mut solutions = [0; 128];

    for i in 0..numbers.len() {
        if numbers[i] == i32::MAX {
            break;
        }
        for j in i + 1..numbers.len() {
            if numbers[j] == i32::MAX {
                break;
            }

            {
                let result = numbers[i] + numbers[j];

                let mut new_numbers = numbers;
                new_numbers[i] = result;
                for k in i + 1..new_numbers.len() - 1 {
                    new_numbers[k] = new_numbers[k + 1];
                }
                new_numbers[new_numbers.len() - 1] = i32::MAX;

                let inner_solutions = step(new_numbers);
                for i in 0..solutions.len() {
                    solutions[i] |= inner_solutions[i];
                }

                if (100..1000).contains(&result) {
                    let index = usize::try_from(result - 100).unwrap();
                    solutions[index/8] |= 1 << (index % 8);
                }
            }

            {
                let result = numbers[i] * numbers[j];

                let mut new_numbers = numbers;
                new_numbers[i] = result;
                for k in i + 1..new_numbers.len() - 1 {
                    new_numbers[k] = new_numbers[k + 1];
                }
                new_numbers[new_numbers.len() - 1] = i32::MAX;

                let inner_solutions = step(new_numbers);
                for i in 0..solutions.len() {
                    solutions[i] |= inner_solutions[i];
                }

                if (100..1000).contains(&result) {
                    let index = usize::try_from(result - 100).unwrap();
                    solutions[index/8] |= 1 << (index % 8);
                }
            }

            {
                let result = numbers[i] - numbers[j];
                if result >= 0 {
                    let mut new_numbers = numbers;
                    new_numbers[i] = result;
                    for k in i + 1..new_numbers.len() - 1 {
                        new_numbers[k] = new_numbers[k + 1];
                    }
                    new_numbers[new_numbers.len() - 1] = i32::MAX;

                    let inner_solutions = step(new_numbers);
                    for i in 0..solutions.len() {
                        solutions[i] |= inner_solutions[i];
                    }

                    if (100..1000).contains(&result) {
                        let index = usize::try_from(result - 100).unwrap();
                        solutions[index/8] |= 1 << (index % 8);
                    }
                }
            }

            {
                let result = numbers[j] - numbers[i];
                if result >= 0 {
                    let mut new_numbers = numbers;
                    new_numbers[i] = result;
                    for k in i + 1..new_numbers.len() - 1 {
                        new_numbers[k] = new_numbers[k + 1];
                    }
                    new_numbers[new_numbers.len() - 1] = i32::MAX;

                    let inner_solutions = step(new_numbers);
                    for i in 0..solutions.len() {
                        solutions[i] |= inner_solutions[i];
                    }

                    if (100..1000).contains(&result) {
                        let index = usize::try_from(result - 100).unwrap();
                        solutions[index/8] |= 1 << (index % 8);
                    }
                }
            }

            {
                if numbers[j] != 0 && numbers[i] % numbers[j] == 0 {
                    let result = numbers[i] / numbers[j];

                    let mut new_numbers = numbers;
                    new_numbers[i] = result;
                    for k in i + 1..new_numbers.len() - 1 {
                        new_numbers[k] = new_numbers[k + 1];
                    }
                    new_numbers[new_numbers.len() - 1] = i32::MAX;

                    let inner_solutions = step(new_numbers);
                    for i in 0..solutions.len() {
                        solutions[i] |= inner_solutions[i];
                    }

                    if (100..1000).contains(&result) {
                        let index = usize::try_from(result - 100).unwrap();
                        solutions[index/8] |= 1 << (index % 8);
                    }
                }
            }

            {
                if numbers[i] != 0 && numbers[j] % numbers[i] == 0 {
                    let result = numbers[j] / numbers[i];

                    let mut new_numbers = numbers;
                    new_numbers[i] = result;
                    for k in i + 1..new_numbers.len() - 1 {
                        new_numbers[k] = new_numbers[k + 1];
                    }
                    new_numbers[new_numbers.len() - 1] = i32::MAX;

                    let inner_solutions = step(new_numbers);
                    for i in 0..solutions.len() {
                        solutions[i] |= inner_solutions[i];
                    }

                    if (100..1000).contains(&result) {
                        let index = usize::try_from(result - 100).unwrap();
                        solutions[index/8] |= 1 << (index % 8);
                    }
                }
            }
        }
    }
    solutions
}
