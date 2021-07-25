use std::io;

use rand::rngs::ThreadRng;
use rand::Rng;

struct Exercise {
    exercise: String,
    solution: String,
}

#[derive(Eq, PartialEq)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

fn main() {
    let number_of_exercises: i8 = ask_for_amount_of_exercises();
    let exercises: Vec<Exercise> = create_exercises(number_of_exercises);

    let mut answers: Vec<String> = Vec::new();

    for e in exercises.iter() {
        println!("Oefening: {}", e.exercise);

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                answers.push(input);
            }
            Err(e) => println!("Er is iets mis gegaan: {}", e),
        }
    }

    let foo = exercises
        .iter()
        .enumerate()
        .map(|(index, exercise)| (index, exercise))
        .collect::<Vec<_>>();

    for f in foo.iter() {
        println!(
            "{}. Oefening: {} - Oplossing: {} - Antwoord: {}",
            f.0, f.1.exercise, f.1.solution, answers[f.0]
        );
    }
}

fn ask_for_amount_of_exercises() -> i8 {
    println!("Hoeveel oefeningen wil je maken?");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Wrong Number");

    let input = input.trim().to_string();

    return input.parse::<i8>().unwrap();
}

fn create_exercises(amount_of_exercises: i8) -> Vec<Exercise> {
    let mut exercises: Vec<Exercise> = Vec::new();
    let mut random_number_generator: ThreadRng = rand::thread_rng();
    let mut counter: i8 = 0;

    loop {
        let left_number: i16 = random_number_generator.gen_range(0..21);
        let right_number: i16 = random_number_generator.gen_range(0..21);

        let random_operation_index: i8 = random_number_generator.gen_range(0..4);

        println!("left: {} - right: {} - operation: {}", left_number, right_number, random_operation_index);

        let random_operation: Operation = match random_operation_index {
            0 => Operation::Addition,
            1 => Operation::Subtraction,
            2 => Operation::Multiplication,
            3 => Operation::Division,
            _ => Operation::Addition,
        };

        let skip_exercise: bool = verify_exercise(&random_operation, left_number, right_number);

        if skip_exercise {
            continue;
        };

        let solution: i16 = match random_operation {
            Operation::Addition => left_number + right_number,
            Operation::Subtraction => left_number - right_number,
            Operation::Multiplication => left_number * right_number,
            Operation::Division => left_number / right_number,
        };

        let exercise: String = match random_operation {
            Operation::Addition => format!("{} + {}", left_number, right_number),
            Operation::Subtraction => format!("{} - {}", left_number, right_number),
            Operation::Multiplication => format!("{} x {}", left_number, right_number),
            Operation::Division => format!("{} : {}", left_number, right_number),
        };

        exercises.push(Exercise { exercise, solution: solution.to_string() });

        if counter == amount_of_exercises {
            return exercises;
        }

        counter += 1;
    }
}

fn verify_exercise(operation: &Operation, left_number: i16, right_number: i16) -> bool {
    match operation {
        Operation::Addition => left_number + right_number > 20,
        Operation::Subtraction => left_number - right_number < 0,
        Operation::Multiplication => left_number * right_number > 20,
        Operation::Division => right_number == 0 || left_number % right_number != 0,
    }
}