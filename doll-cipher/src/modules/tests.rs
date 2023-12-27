/*
DOLL-CIPHER by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// We import the method
/// to generate a key.
use super::key::gen_key;

/// We import the enum that
/// contains key strength variants.
use super::key::KeyLength;


/// We import the method to encrypt
/// a phrase.
use super::encrypt::encrypt;

/// We import the method to encrypt
/// a word.
use super::encrypt::encrypt_word;

/// We import the method to
/// encrypt a letter.
use super::encrypt::encrypt_letter;


/// We import the method to decrypt
/// a phrase.
use super::decrypt::decrypt;

/// We import the method to decrypt
/// a word.
use super::decrypt::decrypt_word;

/// We import the method to decrypt
/// a letter.
use super::decrypt::decrypt_letter;


/// We import the standard length
/// for the highest key security
/// level.
use super::constants::IRON_KEYLENGTH;

/// We import the standard length
/// for the lowest key security
/// level.
use super::constants::SECURE_KEYLENGTH;

// We import the standard length
/// for mid-level key security.
use super::constants::SUPER_SECURE_KEYLENGTH;

/* KEY GENERATION TESTS */

/// Testing key generation with the lowest key security level.
#[test]
pub fn test_key_generation_secure() {
    let key: String = gen_key(&KeyLength::Secure).unwrap();
    assert_eq!(key.len(), (SECURE_KEYLENGTH as usize));
    println!(
        "Key: {}",
        key
    );
}

/// Testing key generation with mid-level key security.
#[test]
pub fn test_key_generation_super_secure() {
    let key: String = gen_key(&KeyLength::SuperSecure).unwrap();
    assert_eq!(key.len(), (SUPER_SECURE_KEYLENGTH as usize));
    println!(
        "Key: {}",
        key
    );
}

/// Testing key generation with the highest key security level.
#[test]
pub fn test_key_generation_iron() {
    let key: String = gen_key(&KeyLength::Iron).unwrap();
    assert_eq!(key.len(), (IRON_KEYLENGTH as usize));
    println!(
        "Key: {}",
        key
    );
}

/* ENCRYPTION TESTS */

/// Testing letter encryption.
#[test]
pub fn test_letter_encryption(){
    let key: String = "84d0lx84t0hqk45jh07g7qwfmpeoyv1igr097ptfnf4415s92k5wfqxjihkheqmocbdb6zo9e0pzhedymeadzp2zq0z0fxjhe192r7a5hvbbunemaa61vb9izo4tx7r8h6vhai6jd43wosbjydkbsvt37p7vut8gsxrsgilq387sptrnmestdt9m8ukmvd0046nq0tcesnkgdax5v6ecmywfc1m0k1ttq7rxhcgq6l3084l3tfioswmvu7nra92h".to_string();
    let encrypted_char: String = encrypt_letter(&'C', &key).unwrap();
    assert_eq!(
        encrypted_char,
        String::from("173124")
    );
}

/// Testing word encryption.
#[test]
pub fn test_word_encryption(){
    let key: String = "84d0lx84t0hqk45jh07g7qwfmpeoyv1igr097ptfnf4415s92k5wfqxjihkheqmocbdb6zo9e0pzhedymeadzp2zq0z0fxjhe192r7a5hvbbunemaa61vb9izo4tx7r8h6vhai6jd43wosbjydkbsvt37p7vut8gsxrsgilq387sptrnmestdt9m8ukmvd0046nq0tcesnkgdax5v6ecmywfc1m0k1ttq7rxhcgq6l3084l3tfioswmvu7nra92h".to_string();
    let encrypted_word: String = encrypt_word(&String::from("Hello"), &key).unwrap();
    assert_eq!(encrypted_word, String::from("461664.288540.692496.692496.865620"));
}

/// Testing phrase encryption.
#[test]
pub fn test_encryption(){
    let key: String = "84d0lx84t0hqk45jh07g7qwfmpeoyv1igr097ptfnf4415s92k5wfqxjihkheqmocbdb6zo9e0pzhedymeadzp2zq0z0fxjhe192r7a5hvbbunemaa61vb9izo4tx7r8h6vhai6jd43wosbjydkbsvt37p7vut8gsxrsgilq387sptrnmestdt9m8ukmvd0046nq0tcesnkgdax5v6ecmywfc1m0k1ttq7rxhcgq6l3084l3tfioswmvu7nra92h".to_string();
    let encrypted_phrase: String = encrypt(&String::from("Hello World"), &key).unwrap();
    assert_eq!(encrypted_phrase, String::from("461664.288540.692496.692496.865620\n1327284.865620.1038744.692496.230832"));
}

/* DECRYPTION TESTS */

/// Testing letter decryption.
#[test]
pub fn test_letter_decryption(){
    let key: String = "84d0lx84t0hqk45jh07g7qwfmpeoyv1igr097ptfnf4415s92k5wfqxjihkheqmocbdb6zo9e0pzhedymeadzp2zq0z0fxjhe192r7a5hvbbunemaa61vb9izo4tx7r8h6vhai6jd43wosbjydkbsvt37p7vut8gsxrsgilq387sptrnmestdt9m8ukmvd0046nq0tcesnkgdax5v6ecmywfc1m0k1ttq7rxhcgq6l3084l3tfioswmvu7nra92h".to_string();
    let encrypted_letter: String = String::from("173124");
    let decrypted_char: char = decrypt_letter(&encrypted_letter, &key).unwrap();
    assert_eq!(
        decrypted_char,
        'c'
    );
}

/// Testing word decryption.
#[test]
pub fn test_word_decryption(){
    let key: String = "84d0lx84t0hqk45jh07g7qwfmpeoyv1igr097ptfnf4415s92k5wfqxjihkheqmocbdb6zo9e0pzhedymeadzp2zq0z0fxjhe192r7a5hvbbunemaa61vb9izo4tx7r8h6vhai6jd43wosbjydkbsvt37p7vut8gsxrsgilq387sptrnmestdt9m8ukmvd0046nq0tcesnkgdax5v6ecmywfc1m0k1ttq7rxhcgq6l3084l3tfioswmvu7nra92h".to_string();
    let encrypted_word: String = String::from("461664.288540.692496.692496.865620");
    let decrypted_word: String = decrypt_word(&encrypted_word, &key).unwrap();
    assert_eq!(decrypted_word, String::from("hello"));
}

/// Testing phrase decryption.
#[test]
pub fn test_decryption(){
    let key: String = "84d0lx84t0hqk45jh07g7qwfmpeoyv1igr097ptfnf4415s92k5wfqxjihkheqmocbdb6zo9e0pzhedymeadzp2zq0z0fxjhe192r7a5hvbbunemaa61vb9izo4tx7r8h6vhai6jd43wosbjydkbsvt37p7vut8gsxrsgilq387sptrnmestdt9m8ukmvd0046nq0tcesnkgdax5v6ecmywfc1m0k1ttq7rxhcgq6l3084l3tfioswmvu7nra92h".to_string();
    let encrypted_phrase: String = String::from("461664.288540.692496.692496.865620\n1327284.865620.1038744.692496.230832");
    let decrypted_phrase: String = decrypt(&encrypted_phrase, &key).unwrap();
    assert_eq!(decrypted_phrase, String::from("hello\nworld"));
}
