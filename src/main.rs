use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Угадай число!");

    let random_number:u8 = rand::thread_rng()
        .gen_range(1..=10);

    loop {
        println!("\nУкажите своё предположение: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Не удалось считать значение");

        let guess: u8 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Введите число от 1 до 10.");
                continue;
            },
        };

        println!("Вы предположили: {guess}");
        // println!("Случайное число: {}", &random_number);
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Вы выбрали меньше."),
            Ordering::Greater => println!("Вы выбрали больше."),
            Ordering::Equal => {
                println!("Вы угадали! Числом было {}.", random_number);
                break;
            }
        }
    }
}
