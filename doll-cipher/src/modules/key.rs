/*
DOLL-CIPHER by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the function
/// from the "rand" crate to
/// generate random numbers in
/// a range.
use rand::thread_rng;

/// We import the error
/// structure for catching
/// and processing errors.
use super::err::DCErr;

/// Needs to be imported to choose
/// randomly from a vector.
use rand::seq::SliceRandom;

/// Importing the method
/// to detect the position of a letter
/// in the alphabet.
use super::utils::get_letter_pos;

/// We import the standard length
/// for the highest key security
/// level.
use super::constants::IRON_KEYLENGTH;

/// We import the standard length
/// for the lowest key security
/// level.
use super::constants::SECURE_KEYLENGTH;

/// We import the standard length
/// for mid-level key security.
use super::constants::SUPER_SECURE_KEYLENGTH;

/// The enum containing the variants
/// of key lengths.
pub enum KeyLength{
    Iron,
    Secure,
    SuperSecure
}

/// Returns a random sequence of numbers and letters as an encryption key.
/// If this fails, an error is returned.
pub fn gen_key(sec_level: &KeyLength) -> Result<String, DCErr> {
    let digits: Vec<char> = "1234567890abcdefghijklmnopqrstuvwxyz".to_string().chars().collect();
    let mut res: Vec<char> = Vec::new();
    let kl: u32;
    match sec_level {
        KeyLength::Secure => kl = SECURE_KEYLENGTH,
        KeyLength::SuperSecure => kl = SUPER_SECURE_KEYLENGTH,
        KeyLength::Iron => kl = IRON_KEYLENGTH
    }
    for _i in 1..(kl+1){
        let res_char: char = match digits.choose(&mut thread_rng()){
            Some(res_char) => *res_char,
            None => {
                let e: String = format!("Could not generate key.");
                return Err::<String, DCErr>(DCErr::new(&e.to_string()));
            }
        };
        res.push(
            res_char
        );
    }
    let result: String = res.iter().collect();
    Ok(result)
}

/// Returns the sum of all digits in the key and the sum of the positions
/// of letters in the key. If this fails, an error is returned.
pub fn get_key_sum(key: &String) -> Result<u32, DCErr> {
    let mut sum: u32 = 0;
    for i in key.chars().collect::<Vec<char>>(){
        if i.is_alphabetic(){
            let alphabet_num: u32 = get_letter_pos(&i);
            sum += alphabet_num;
        }
        else {
            let num: u32 = match i.to_string().parse(){
                Ok(num) => num,
                Err(e) => {
                    return Err::<u32, DCErr>(DCErr::new(&e.to_string()));
                }
            };
            sum += num;
        }
    }
    Ok(sum)
}
