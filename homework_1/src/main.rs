// assignment 1
const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}

fn assignment1_temperature_converter() {
    println!("Temperature Converter ");
    let mut temp_f: f64 = 32.0;

    println!("{}°F is {:.2}°C", temp_f, fahrenheit_to_celsius(temp_f));

    for _ in 0..5 {
        temp_f += 1.0;
        println!("{}°F is {:.2}°C", temp_f, fahrenheit_to_celsius(temp_f));
    }
    println!();
}

// assignment 2

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn assignment2_number_analyzer() {
    println!("Number Analyzer");
    let numbers = [12, 5, 7, 15, 20, 9, 3, 8, 25, 30];

    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{} → FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{} → Fizz", num);
        } else if num % 5 == 0 {
            println!("{} → Buzz", num);
        } else if is_even(num) {
            println!("{} → Even", num);
        } else {
            println!("{} → Odd", num);
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    let mut largest = numbers[0];
    let mut j = 1;
    loop {
        if j >= numbers.len() {
            break;
        }
        if numbers[j] > largest {
            largest = numbers[j];
        }
        j += 1;
    }
    println!("Largest number: {}", largest);
    println!();
}

// assignment 3

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}


fn assignment3_guessing_game() {
    println!("Guessing Game");
    let secret: i32 = 15; // hard-coded secret number
    let mut guess: i32 = 5; // starting guess
    let mut attempts = 0;

    loop {
        attempts += 1;
        let result = check_guess(guess, secret);

        if result == 0 {
            println!("{} is correct! 🎉", guess);
            break;
        } else if result == 1 {
            println!("{} is too high!", guess);
        } else {
            println!("{} is too low!", guess);
        }

        // Simulate changing guesses
        guess += 2;
    }

    println!("It took {} guesses to find the number.", attempts);
    println!();
}

fn main() {
    assignment1_temperature_converter();
    assignment2_number_analyzer();
    assignment3_guessing_game();
}