use std::convert::TryFrom;
use std::collections::HashMap;
use std::cell::RefCell;


fn generate_small_numbers(mut numbers: [i32; 6], index: usize, max_number: i32) {
    // TODO FIXME don't use at all?

    for x in (2..max_number).rev() {
        if x % 2 == 0 && numbers[index-1] != x { continue; }
        
        // 1 = 2, 3
        // 10 = 20, 21
        
        numbers[index] = x/2;
        
        if index == 5 {
            // TODO call function
            println!("{:?}", numbers);
        } else {
            generate_small_numbers(numbers, index + 1, x-1);
        }
    }
}

fn generate_numbers() {
    // TODO CHECK
    const MAJOR_NUMBERS: [[i32; 6]; 15] = [
        [ 25, 0,  0,  0,  0,  0, ],
        [ 50, 0,  0,  0,  0,  0, ],
        [ 75 ,0,  0,  0,  0,  0, ],
        [ 100,0,  0,  0,  0,  0, ],
        [ 25, 50, 0,  0,  0,  0, ],
        [ 25, 75, 0,  0,  0,  0, ],
        [ 25, 100,0,  0,  0,  0, ],
        [ 50, 75, 0,  0,  0,  0, ],
        [ 50, 100,0,  0,  0,  0, ],
        [ 75, 100,0,  0,  0,  0, ],
        [ 25, 50, 75, 0,  0,  0, ],
        [ 25, 50, 100,0,  0,  0, ],
        [ 25, 75, 100,0,  0,  0, ],
        [ 50, 75, 100,0,  0,  0, ],
        [ 25, 50, 75, 100,0,  0, ]
    ];

    // TODO CHECK
    const INDEX: [usize; 15] = [1,1,1,1,2,2,2,2,2,2,3,3,3,3,4];
    
    for i in 0..MAJOR_NUMBERS.len() {
        generate_small_numbers(MAJOR_NUMBERS[i], INDEX[i], 22);
    }
}


thread_local! {
    pub static MEMOIZATION: RefCell<HashMap<[i32; 6], u32>> = RefCell::new(HashMap::new());
}

fn main() {
    //println!("Hello, world!");
    //generate_numbers()
    let mut solutions = [false; 900];
    step(&mut solutions, [1, 3, 5, 25, 50, 100]);
    
    MEMOIZATION.with(|m| {
         for (key, value) in &(*m.borrow()) {
            println!("{:?}: {}", key, value);
        }
    });
    
    for (i, solution) in solutions.iter().enumerate() {
        if !*solution {
            println!("{}", i+100)
        }
    }
    
    generate_numbers();
}

// log2(100*75*50*25*10*10) = 30
// u32::MAX means empty as is shouldn't be reachable

fn step(solutions: &mut [bool; 900], numbers: [i32; 6]) {
    MEMOIZATION.with(|m| {
         *m.borrow_mut().entry(numbers).or_insert(0) += 1;
    });
    
    for i in 0..numbers.len() {
        if numbers[i] == i32::MAX { continue };
        for j in i+1..numbers.len() {
            if numbers[j] == i32::MAX { continue; }
            
            {
                let result = numbers[i] + numbers[j];
            
                let mut new_numbers = numbers;
                new_numbers[i] = result;
                new_numbers[j] = i32::MAX;
                
                if (100..1000).contains(&result) {
                    solutions[usize::try_from(result-100).unwrap()] = true;
                }
                
                step(solutions, new_numbers);
            }
            
            {
                let result = numbers[i] * numbers[j];
                
                let mut new_numbers = numbers;
                new_numbers[i] = result;
                new_numbers[j] = i32::MAX;
                
                if (100..1000).contains(&result) {
                    solutions[usize::try_from(result-100).unwrap()] = true;
                }
                
                step(solutions, new_numbers);
            }
            
            {
                let result = numbers[i] - numbers[j];
                if result >= 0 {
                    let mut new_numbers = numbers;
                    new_numbers[i] = result;
                    new_numbers[j] = i32::MAX;
                    
                    if (100..1000).contains(&result) {
                        solutions[usize::try_from(result-100).unwrap()] = true;
                    }
                    
                    step(solutions, new_numbers);
                }
            }
            
            {
                let result = numbers[j] - numbers[i];
                if result >= 0 {
                    let mut new_numbers = numbers;
                    new_numbers[i] = result;
                    new_numbers[j] = i32::MAX;
                    
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
                    new_numbers[j] = i32::MAX;
                    
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
                    new_numbers[j] = i32::MAX;
                    
                    if (100..1000).contains(&result) {
                        solutions[usize::try_from(result-100).unwrap()] = true;
                    }
                    
                    step(solutions, new_numbers);
                }
            }
        }
    }
}



