// generator.rs

use rand::Rng;

pub fn gen_random_lower() -> char {
    /* Generates random lowercase alphanumeric
     * doesn't include J, U, or W because the original
     * Latin alphabet didn't have them (source: brittanica.com)
     */

    let charset: &str = "abcdefghiklmnopqrstvxyz";
    let charset_len: usize = charset.chars().count();

    let rng: rand::rngs::ThreadRng = rand::thread_rng();
    let index = rand::thread_rng().gen_range(0..charset_len);

    charset.as_bytes()[index] as char

}

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
    // about 38% (5/13) of characters are vowels
    // median word length is 8
    // estimated mean character count between vowels is 1.333-

    ret
}

fn gen_random_sentence(length: u8) -> String {
    "Lorem Ipsum Dolor sit amet, consetetur adipiscing elit.".to_string()
}