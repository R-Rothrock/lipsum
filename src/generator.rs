// generator.rs

use rand;

fn gen_random_lower() -> char {'?'}

fn gen_random_lower_vowel() -> char {'?'}

fn gen_random_upper() -> char {'?'}

fn gen_random_upper_vowel() -> char {'?'}

fn gen_random_word(capitalized: bool) -> String {
    /* Generates random word.
     */

    // sample sentence from which I derived my numbers:
    // Romani partem princepes gallorum occiderunt.
    // The Romans killed part of the leading men of the Gauls.

    let mut ret: String = String::new();
    
    // RULES
    // 5/13 of characters are vowels
    // mean word length is 7.8
    // median word length is 8

    ret
}

fn gen_random_sentence(length: u8) -> String {
    "Lorem Ipsum Dolor sit amet, consetetur adipiscing elit.".to_string()
}