// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    let mut result: Vec<String> = vec!["-".to_string(); secret_word_chars.len()];
    println!("Welcome to CS110L Hangman!");
    let mut guess_chars: String = String::from("");;
    let mut chance: i32 = 5;
    let mut flag:bool = true;
    while  true {
        flag = false;
        let joined_string: String = result.join("");
        println!("The word so far is {}",joined_string.trim());
        println!("You have guessed the following letters: {}",guess_chars);
        println!("You have {} guesses left",chance);
        println!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        guess = guess.trim().to_string();
        for (i, &char_in_word) in secret_word_chars.iter().enumerate() {
            if let Some(ch) = guess.chars().next() {
                if ch == char_in_word {
                    result[i] = guess.clone();
                    flag = true;
                }
            }}
            if !joined_string.contains('-'){
                println!("Congratulations you guessed the secret word: {}",secret_word);
            }
            if chance == 0{
                println!("Sorry, you ran out of guesses!")
            }


        guess_chars.push(guess.chars().next().unwrap());

        if !flag {
            println!("Sorry, that letter is not in the word");
            chance -= 1;
        }
    }


    // Uncomment for debugging:
    // println!("random word: {}", secret_word);
    // Your code here! :)
}
