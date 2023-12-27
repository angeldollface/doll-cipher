/*
DOLL-CIPHER by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// We import the error
/// structure for catching
/// and processing errors.
use super::err::DCErr;

/// Returns the position of a letter in the
/// alphabet as an integer.
pub fn get_letter_pos(letter: &char) -> u32 {
    let mut result: u32 = 0;
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".to_string().chars().collect();
    for i in 1..alphabet.len(){
        if letter == &alphabet[i]{
            result = (i as u32) + 1;
        }
        else {}
    }
    result 
}

/// Returns the letter at the position of the supplied integer. If this
/// fails, an error is returned.
pub fn get_letter_from_pos(pos: &u32) -> Result<char,DCErr> {
    let mut res: char = '*';
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".to_string().chars().collect();
    for i in 1..alphabet.len() {
        if ((*pos as usize)-1) == i {
            res = alphabet[i];
        }
        else {}
    }
    if res == '*'{
        let e: String = format!("Could not get letter for integer: \"{}\"!", pos);
        return Err::<char, DCErr>(DCErr::new(&e.to_string()));
    }
    Ok(res)
}

// Returns the words in a phrase as a string vector.
pub fn get_words_from_phrase(msg: &String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for line in msg.split("\n"){
        lines.push(line.to_string());
    }
    let mut result: Vec<String> = Vec::new();
    for line in lines {
        for word in line.split(" "){
            result.push(word.to_string());
        }
    }
    result
}

/// Returns an obfuscated letter as a numerical string. If this fails, an error
/// is returned.
pub fn obfuscate_letter(letter_pos: &u32, key_sum: &u32) -> Result<String, DCErr>{
    let digit_sum = match sum_number_digits(&key_sum){
        Ok(digit_sum) => digit_sum,
        Err(e) => {
            return Err::<String, DCErr>(DCErr::new(&e.to_string()));
        }
    };
    Ok(((letter_pos * key_sum) * digit_sum).to_string())
}

/// Deobfuscates a letter and returns the positon of a character in the alphabet. 
/// If this fails, an error is returned.
pub fn deobfuscate_letter(obfuscated: &u32, key_sum: &u32) -> Result<u32, DCErr> {
    let digit_sum = match sum_number_digits(&key_sum){
        Ok(digit_sum) => digit_sum,
        Err(e) => {
            return Err::<u32, DCErr>(DCErr::new(&e.to_string()));
        }
    };
    Ok((obfuscated / digit_sum) / key_sum)
}

/// Atempts to return the sum of all digits in a number.
pub fn sum_number_digits(number: &u32) -> Result<u32, DCErr> {
    let num_string: Vec<char> = number.to_string().chars().collect();
    let mut sum: u32 = 0;
    for character in num_string {
        let num: u32 = match character.to_string().parse(){
            Ok(num) => num,
            Err(e) => {
                return Err::<u32, DCErr>(DCErr::new(&e.to_string()));
            }
        };
        sum += num
    }
    Ok(sum)
}