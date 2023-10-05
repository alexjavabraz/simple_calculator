use core::num;
use std::{io, os::unix::process};
use rand::prelude::*;

fn main() {
    let banner =
    "Digite um numero \n
     entre 1 e 100 \n
     e tente acertar \
    ";
    println!("{:-^40}", "Try to discover game");
    println!("{}", "-".repeat(40));
    println!("{banner}");

    let mut number = thread_rng().gen_range(1..10);

    let mut buffer = String::new();
    let mut number_choosed_by_user: i32 = 0;

    while !validate_numbers(number_choosed_by_user, number) {
        number = thread_rng().gen_range(1..10);
        println!("Try discover the secret number between 1 and 10: ");
        buffer = String::new();
        io::stdin().read_line(&mut buffer);
        number_choosed_by_user = buffer.trim().parse().unwrap();
    }
    
    
    println!("Generated number is {}", number);
}

fn validate_numbers(number_choosed_by_user: i32, number : i32) -> bool {

    if number_choosed_by_user == 0 {
         return false;
    }

    if number_choosed_by_user > number {
        println!("too high, the correct is {}", number);
        return false;
    } else if number_choosed_by_user < number {
        println!("too slow, the correct is {}", number);
        return false;
    } else if number_choosed_by_user == number {
        println!("correct, the number is {}", number);
    }
    true
}

fn mainErrorPanic() {
    let mut buffer = String::new();
    println!("Enter a message:");
    io::stdin().read_line(&mut buffer);
    print!("buffer is {} ", buffer);

    let number2: i32 = buffer.trim().parse().unwrap();
    println!("number2 is {}", number2 + 1);

    let number = buffer.trim().parse::<i32>();
    println!("number is {:?}", number);
}

fn mainoldold() {
    let message = String::from("Greetings from Earth");
    println!("message is {}", message);

    let last_word = &message[10..];
    println!("last word is {}", last_word);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets: &[i32] = &planets[4..];
    print!("inner_plannets are {:?}", inner_planets);
}

fn mainold() {
    let mut rocket_fuel = process_fuel();
    println!("Checking the fuel {} and the length is {} ", rocket_fuel, rocket_fuel.len());
    rocket_fuel.push_str("test");
    println!("Checking the fuel now {}", rocket_fuel);
}

fn process_fuel() -> String {
    let propellant = String::from("Gas");
    propellant
}

fn numeros() {
    let numbers = [1,9,-2,0,23,20,-7,13,37,20,56,-18,20,3];
    let mut max: i32 = 0;
    let mut min: i32 = 0;
    let mut mean: f64 = 0.0;

    let size = numbers.len() as i32;
    let mut total : i32 = 0;

    for number in numbers {
        if number < max {
            max = number;
        }
        if number < min {
            min = number;
        }

        total += number;

    }

    mean = (total / size) as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");

}

fn calculate() {
    let banner =
        "Digite uma sequencia de numeros \n
         separado por virgula \n
         exemplo: 1,2,3,45 \
        ";
    println!("{:-^40}", "Calculadora");
    println!("{}", "-".repeat(40));
    println!("{banner}");

    let mut s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");

    println!("Voce digitou {}", s.to_uppercase().trim()); // try digit some emoji 🖖 see the complete list here https://unicode.org/emoji/charts/full-emoji-list.html
    println!("Quantidade de letras {}", s.trim().chars().count());

    fn verify (mut item: &str) -> &str {
        println!("Verifying {:?}", item);
        if item.is_empty() {
            item = "0";
        }
        if item == "\n" {
            item = "0";
        }
        item
    }

    let nums: Vec<i32> =
        s.split(",")
            .map(verify)
            .map(|c| c.trim().parse().expect("error")) // closure
            .collect();
    println!("Voce digitou {:?}", nums);

    let result: i32 = nums.iter().sum();
    println!("O total é {} ", result);

    println!("{}", "-".repeat(40));
}
