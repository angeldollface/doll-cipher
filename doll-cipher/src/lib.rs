/*
DOLL-CIPHER by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the CLI module.
pub use modules::cli::*;

/// Re-exporting the key generation
/// module.
pub use modules::key::*;

/// Re-exporting the error
/// module.
pub use modules::err::*;

/// Re-exporting the
/// module with useful
/// functions.
pub use modules::utils::*;

/// Re-exporting the decryption
/// module.
pub use modules::decrypt::*;

/// Re-exporting the encryption
/// module.
pub use modules::encrypt::*;

/// Re-exporting the
/// module with static constants.
pub use modules::constants::*;