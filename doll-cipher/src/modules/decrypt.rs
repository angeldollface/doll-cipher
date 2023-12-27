/*
DOLL-CIPHER by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// We import the error
/// structure for catching
/// and processing errors.
use super::err::DCErr;

/// Importing the method to
/// compute the key sum.
use super::key::get_key_sum;

/// Importing the method to reverse
/// the obfuscation of a letter.
use super::utils::deobfuscate_letter;

/// Importing the method to retrieve
/// the letter at a certain position in the
/// alphabet.
use super::utils::get_letter_from_pos;

/// Returns a decrypted letter. If it fails, an error is returned.
pub fn decrypt_letter(
    encrypted_char: &String, 
    key: &String
) -> Result<char, DCErr> {
    let parsed_char: u32 = match encrypted_char.parse::<u32>(){
        Ok(parsed_char) => parsed_char,
        Err(e) => {
            return Err::<char, DCErr>(DCErr::new(&e.to_string()));  
        }
    };
    let key_sum: u32 = match get_key_sum(key){
        Ok(key_sum) => key_sum,
        Err(e) => {
            return Err::<char, DCErr>(DCErr::new(&e.to_string()));
        }
    };
    let letter_pos: u32 = match deobfuscate_letter(&parsed_char, &key_sum){
        Ok(letter_pos) => letter_pos,
        Err(e) => {
            return Err::<char, DCErr>(DCErr::new(&e.to_string()));
        }
    };
    let result: char = match get_letter_from_pos(&letter_pos){
        Ok(result) => result,
        Err(e) => {
            return Err::<char, DCErr>(DCErr::new(&e.to_string()));
        }
    };
    Ok(result)
}

/// Returns a decrypted word. If it fails, an error is returned.
pub fn decrypt_word(
    encrypted_word: &String, 
    key: &String
) -> Result<String,DCErr> {
    let mut decrypted_letters: Vec<char> = Vec::new();
    let letter_iterator = encrypted_word.split(".");
    for letter in letter_iterator {
        let decrypted_letter: char = match decrypt_letter(&(letter.to_string()), &key){
            Ok(decrypted_letter) => decrypted_letter,
            Err(e) => {
                return Err::<String, DCErr>(DCErr::new(&e.to_string()));
            }
        };
        decrypted_letters.push(decrypted_letter);
    }
    let result: String = decrypted_letters.into_iter().collect();
    Ok(result)
}

/// Returns a decrypted phrase. If it fails, an error is returned.
pub fn decrypt(
    encrypted_msg: &String, 
    key: &String
) -> Result<String,DCErr> {
    let mut decrypted_words: Vec<String> = Vec::new();
    let word_iterator = encrypted_msg.split("\n");
    for word in word_iterator {
        let decrypted_word: String = match decrypt_word(&(word.to_string()), &key){
            Ok(decrypted_word) => decrypted_word,
            Err(e) => {
                return Err::<String, DCErr>(DCErr::new(&e.to_string()));
            }
        };
        decrypted_words.push(decrypted_word);
    }
    let result: String = decrypted_words.join("\n");
    Ok(result)
}

