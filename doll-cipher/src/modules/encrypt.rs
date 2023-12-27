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

/// Importing the method to
/// get the position of a letter
/// in the alphabet.
use super::utils::get_letter_pos;

/// Importing the method to "obfuscate"
/// a letter.
use super::utils::obfuscate_letter;

/// Importing the method to get the
/// words in a phrase.
use super::utils::get_words_from_phrase;

/// Returns an encrypted letter. If it fails, an error is returned.
pub fn encrypt_letter(
    character: &char, 
    key: &String
) -> Result<String, DCErr> {
    if character.to_ascii_lowercase().is_alphabetic(){
        let letter_pos: u32 = get_letter_pos(&character.to_ascii_lowercase());
        let key_sum: u32 = match get_key_sum(key){
            Ok(key_sum) => key_sum,
            Err(e) => {
                return Err::<String, DCErr>(DCErr::new(&e.to_string()));
            }
        };
        let result: String = match obfuscate_letter(&letter_pos, &key_sum){
            Ok(result) => result,
            Err(e) => {
                return Err::<String, DCErr>(DCErr::new(&e.to_string()));
            }
        };
        return Ok(result);
    }
    else {
        let e: String = format!("\"{}\" is an illegal character.", character);
        return Err::<String, DCErr>(DCErr::new(&e.to_string()));
    }
}

/// Returns an encrypted word. If it fails, an error is returned.
pub fn encrypt_word(
    msg: &String, 
    key: &String
) -> Result<String, DCErr> {
    let mut encrypted_chars: Vec<String> = Vec::new();
    let msg_chars: Vec<char> = msg.chars().collect();
    for msg_char in msg_chars{
        let encrypted_letter: String = match encrypt_letter(&msg_char, &key){
            Ok(encrypted_letter) => encrypted_letter,
            Err(e) => {
                return Err::<String, DCErr>(DCErr::new(&e.to_string()));
            }
        };
        encrypted_chars.push(encrypted_letter);
    }
    let result: String = encrypted_chars.join(".");
    Ok(result)    
}

/// Returns an encrypted phrase. If it fails, an error is returned.
pub fn encrypt(
    msg: &String, 
    key: &String
) -> Result<String, DCErr> {
    let mut result: Vec<String> = Vec::new();
    let words: Vec<String> = get_words_from_phrase(msg);
    for word in words{
        let encrypted_word: String = match encrypt_word(&word, key){
            Ok(encrypted_word) => encrypted_word,
            Err(e) => {
                return Err::<String, DCErr>(DCErr::new(&e.to_string()));
            }
        };
        result.push(encrypted_word);
    }
    let res: String = result.join("\n");
    Ok(res)
}