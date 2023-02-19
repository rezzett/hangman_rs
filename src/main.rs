use std::collections::HashSet;

use util::get_user_letter;

use crate::util::get_word;

mod util;
fn main() {
    if let Some(word) = get_word("words.txt") {
        let mut word_letters: Vec<char> = word.chars().collect();
        let alphabet: HashSet<char> = ('A'..='Z').into_iter().collect();
        let mut used_letters: Vec<char> = Vec::new();
        let mut tries = 10;

        while word_letters.len() > 0 && tries > 0 {
            let mut current_world = String::new();
            for c in word.chars() {
                if used_letters.contains(&c) {
                    current_world.push(c);
                } else {
                    current_world.push('-');
                }
            }
            println!("Word: {}", current_world);

            for c in &used_letters {
                print!("{} ", c);
            }
            println!("");

            println!("Try: {tries}");
            if let Some(user_letter) = get_user_letter("Enter your guess: ") {
                if alphabet.contains(&user_letter) && !used_letters.contains(&user_letter) {
                    used_letters.push(user_letter);
                    if word_letters.contains(&user_letter) {
                        word_letters.retain(|c| &user_letter != c);
                    } else {
                        println!("The letter isn't in the word.");
                        tries -= 1;
                    }
                } else if used_letters.contains(&user_letter) {
                    println!("You already use this caracter! Please tyr again.");
                } else {
                    println!("Invalid charcter! Please try again.");
                }
            }
        }

        if tries == 0 {
            println!("You loose! The word was {word}");
        } else {
            println!("You win! You guessed the word: {word}");
        }
    }
}
