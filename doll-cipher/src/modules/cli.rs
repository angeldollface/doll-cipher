/*
DOLL-CIPHER by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the structure
/// to make a CLI app from the
/// "cliply" crate.
use cliply::App;

/// Importing the method to 
/// generate a key.
use super::key::gen_key;

/// Importing the error
/// structure from the "cliply"
/// crate to catch errors.
use cliply::CliplyError;

/// We import the enum that
/// contains key strength variants.
use super::key::KeyLength;

/// Importing the method to encrypt
/// a phrase.
use super::encrypt::encrypt;

/// Importing the method to decrypt
/// a phrase.
use super::decrypt::decrypt;

/// This crate's tiny CLI.
pub fn cli() -> () {
    let mut doll_cipher: App = App::new(
        &"Doll Cipher",
        &"0.1.0",
        &"Angel Dollface <angelbbe@proton.me>"
    );
    doll_cipher.add_arg(
        &"encr", 
        &"   encrypts the supplied message with the given key", 
        &"false"
    );
    doll_cipher.add_arg(
        &"decr", 
        &"   decrypts the supplied cipher text with the given key", 
        &"false"
    );
    doll_cipher.add_arg(
        &"genk", 
        &"   generates a cipher key of the supplied strength", 
        &"false"
    );
    doll_cipher.add_arg(
        &"iron", 
        &"   generates a cipher key with a 512-bit length", 
        &"false"
    );
    doll_cipher.add_arg(
        &"msec", 
        &"   generates a cipher key with a 128-bit length", 
        &"false"
    );
    doll_cipher.add_arg(
        &"ssec", 
        &"   generates a cipher key with a 256-bit length", 
        &"false"
    );
    doll_cipher.add_arg(
        &"ckey", 
        &"    the cipher key to use for encryption/decryption", 
        &"true"
    );
    doll_cipher.add_arg(
        &"ptxt", 
        &"    the text to decrypt/encrypt text", 
        &"true"
    );
    if doll_cipher.version_is() {
        println!("{}", doll_cipher.version_info());
    }
    else if doll_cipher.help_is() {
        println!("{}", doll_cipher.help_info());
    }

    // Generate a key with the lowest security level.
    else if doll_cipher.arg_was_used(&"genk") &&
        doll_cipher.arg_was_used(&"msec") 
    {
        match gen_key(&KeyLength::Secure){
            Ok(key) => {
                println!("{}", &key);
            },
            Err(e) => {
                println!("{}", e);
            }
        };
    }
    
    // Generate a key with mid-level security.
    else if doll_cipher.arg_was_used(&"genk") &&
        doll_cipher.arg_was_used(&"ssec") 
    {
        match gen_key(&KeyLength::SuperSecure){
            Ok(key) => {
                println!("{}", &key);
            },
            Err(e) => {
                println!("{}", e);
            }
        };
    }

    // Generate a key with the highest security level.
    else if doll_cipher.arg_was_used(&"genk") &&
        doll_cipher.arg_was_used(&"iron") 
    {
        match gen_key(&KeyLength::Iron){
            Ok(key) => {
                println!("{}", &key);
            },
            Err(e) => {
                println!("{}", e);
            }
        };
    }

    // Encrypt a message.
    else if doll_cipher.arg_was_used(&"encr") &&
        doll_cipher.arg_was_used(&"ckey") && 
        doll_cipher.arg_was_used(&"ptxt")
    {
        let key: Result<String, CliplyError> = doll_cipher.get_arg_data(&"ckey");
        let text: Result<String, CliplyError> = doll_cipher.get_arg_data(&"ptxt");

        match key {
            Ok(ckey) => {
                match text {
                    Ok(ctxt) => {
                        match encrypt(&ctxt, &ckey){
                            Ok(res) => {
                                println!("{}", &res);
                            },
                            Err(e) => {
                                println!("{}", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        };

    }

    // Decrypt a message.
    else if doll_cipher.arg_was_used(&"decr") &&
        doll_cipher.arg_was_used(&"ckey") && 
        doll_cipher.arg_was_used(&"ptxt")
    {
        let key: Result<String, CliplyError> = doll_cipher.get_arg_data(&"ckey");
        let text: Result<String, CliplyError> = doll_cipher.get_arg_data(&"ptxt");

        match key {
            Ok(ckey) => {
                match text {
                    Ok(ctxt) => {
                        match decrypt(&ctxt, &ckey){
                            Ok(res) => {
                                println!("{}", &res);
                            },
                            Err(e) => {
                                println!("{}", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        };

    }

    else {
        println!("{}", doll_cipher.help_info());
    }
    
}
