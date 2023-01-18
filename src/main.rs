use std::io;

fn main() {

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
