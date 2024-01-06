use std::io;
use rand::Rng;

fn main() {
    println!("Угадай число!");

    let random_number:i8 = rand::thread_rng()
        .gen_range(1..=10);

    println!("Укажите своё предположение: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Не удалось считать значение");

    println!("Вы предположили: {guess}");
    println!("Случайное число: {}", &random_number);
}
