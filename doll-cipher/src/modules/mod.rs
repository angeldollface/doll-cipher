/*
DOLL-CIPHER by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the key
/// generation module.
pub mod key;

/// Declaring the error
/// module.
pub mod err;

/// Declaring the CLI
/// module.
pub mod cli;

/// Declaring the module
/// with useful functions.
pub mod utils;

/// Declaring the module
/// for encryption.
pub mod encrypt;

/// Declaring the module
/// for decryption.
pub mod decrypt;

/// Declaring the module
/// for algorithm constants.
pub mod constants;

/// The testing module.
#[cfg(test)]
pub mod tests;