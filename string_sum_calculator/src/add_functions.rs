pub fn test_add(input: String, result: i32) {
    let add_result: i32 = add(&input);
    
    if add_result == result {
        println!("Test success!");
    } else {
        println!("Test error! {}, {}", add_result, result);
    }

    assert_eq!(add_result, result);
}

pub fn add(numbers_string: &String) -> i32 {
    let extracted_numbers = extract_numbers(numbers_string);
    let sorted_numbers = sort_numbers(extracted_numbers);
    let numbers_sum = sum_numbers(sorted_numbers);
    
    numbers_sum
}

fn extract_numbers(numbers_string: &String) -> Vec<i32> {
    let mut numbers:Vec<i32> = Vec::new();
    let mut number_start_pos: usize = 99;

    for (i, character) in numbers_string.chars().enumerate() {
        if is_char_a_number(&character) || character == '-' {
            if i < numbers_string.len() - 1 && (is_char_a_number(&numbers_string.chars().nth(i + 1).unwrap()) || character == '-') {
                if number_start_pos == 99 {
                    number_start_pos = i;
                }
            } else {
                if number_start_pos != 99 {
                    match numbers_string[number_start_pos..i+1].parse() {
                        Ok(number) => numbers.push(number),
                        Err(error) => println!("Error during multiple digit parse: {}", error),
                    };
                } else {
                    numbers.push(character.to_digit(10).unwrap().try_into().unwrap());
                }
                number_start_pos = 99;
            }
        }
    }

    numbers
}

fn sort_numbers(numbers: Vec<i32>) -> Vec<i32> {
    let mut sorted_numbers: Vec<i32> = Vec::new();
    let mut negative_numbers: Vec<i32> = Vec::new();

    for number in numbers {
        if number < 0 {
            negative_numbers.push(number);
        } else if number >= 0 && number <= 1000 {
            sorted_numbers.push(number);
        }
    }

    if negative_numbers.len() > 0 {
        println!("Negative numbers are not allowed! {:?}", negative_numbers);
    }

    sorted_numbers
}

fn sum_numbers(numbers: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;

    for number in numbers {
        sum += number;
    }

    sum
}

fn is_char_a_number(character: &char) -> bool {
    let is_a_number = character.to_digit(10);

    match is_a_number {
        Some(_) => (0..10).contains(&is_a_number.unwrap()),
        None => false,
    }
}
