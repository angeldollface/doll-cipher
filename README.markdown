# DOLL CIPHER :lock: :dolls:

![GitHub CI](https://github.com/angeldollface/doll-cipher/actions/workflows/rust.yml/badge.svg)

***A Rustacean implementation of an encryption algorithm of my own design. :lock: :dolls:***

## ABOUT :books:

This repository contains an implementation of an algorithm of my own design in Rust. The algorithm attempts to conform with the AES encryption standard and encrypts plain text. This repository contains a library, a CLI tool, and a REST API binary.

## THE ALGORITHM :abacus:

### Steps

Given a phrase, the algorithm executes the following steps:

- 1.) A key of any of the following lengths is generated: 128 bits, 256 bits or 512 bits. The key consists of a random sequence of letters and numbers.

- 2.) The phrase is split up into letters.

- 3.) The position of each letter in the alphabet is calculated.

- 4.) All digits in the key are summed. The position of any letters in the key are located in the alphabet and added to this sum.

- 5.) The position in the alphabet of each letter in the phrase is multiplied by the key sum.

- 6.) The digits in the key sum are summed.

- 7.) The result from step `5` is divided by the digit sum of the key sum.

- 8.) The resulting number represents an encrypted character from the phrase.

- 9.) Each character in each word in the phrase is encrypted in this way. The encrypted characters are separated by dots.

- 9.) Each encrypted word is separated by a `\n` character.

### Example

```Text
Key: 84d0lx84t0hqk45jh07g7qwfmpeoyv1igr097ptfnf4415s92k5wfqxjihkheqmocbdb6zo9e0pzhedymeadzp2zq0z0fxjhe192r7a5hvbbunemaa61vb9izo4tx7r8h6vhai6jd43wosbjydkbsvt37p7vut8gsxrsgilq387sptrnmestdt9m8ukmvd0046nq0tcesnkgdax5v6ecmywfc1m0k1ttq7rxhcgq6l3084l3tfioswmvu7nra92h
```

```Text
Phrase: Hello World
```

```Text
Cipher Text:
461664.288540.692496.692496.865620
1327284.865620.1038744.692496.230832
```

## INSTALLATION :inbox_tray:

## In a Rust project

To use the ***Doll Cipher*** in your Rust project, add this line to your project's `Cargo.toml`:

```TOML
doll-cipher = { git = "https://github.com/angeldollface/doll-cipher", version = "0.1.0" }
```

## As a CLI tool

Make sure you have Rust and Cargo installed. Once both of these tools are installed, you can execute this command from a command prompt:

```bash
cargo install --git https://github.com/angeldollface/doll-cipher --path doll-cipher
```

## The REST API binary

Make sure you have Rust and Cargo installed. Once both of these tools are installed, you can execute this command from a command prompt:

```bash
cargo install --git https://github.com/angeldollface/doll-cipher --path doll-cipher-api
```

## USAGE :gear:

### Rust APIs

To view this crate's APIs, please peruse the `src` directory. 

### CLI Tool

You can use the CLI tool this crate provides like this:

- Get version information:

```bash
dc version
# OR
dc --version
# OR
dc -v
```

- Get help information:

```bash
dc help
# OR
dc --help
# OR
dc -h
```

- Generate a key with the lowest security level:

```bash
# 128-bit key.
dc -g -m
# OR
dc --genk --msec
# OR
dc genk msec
```

- Generate a key with mid-level security:

```bash
# 256-bit key.
dc -g -s
# OR
dc --genk --ssec
# OR
dc genk ssec
```

- Generate a key with the highest security level:

```bash
# 512-bit key.
dc -g -i
# OR
dc --genk --iron
# OR
dc genk iron
```

- Encrypt a message with a key:

```bash
dc -e -c "tkcbxp3ws61g7x6y44wcs68a8h71jjh8adbi3yg7q4e9btrdbqujczirog56ojbauewuihc7tb3veshoczfl0p0c12cmrvk6l111lur77e08wh6oqozxetp32s8kzvu0" -p "Hello World"
# OR
dc --encr --ckey "tkcbxp3ws61g7x6y44wcs68a8h71jjh8adbi3yg7q4e9btrdbqujczirog56ojbauewuihc7tb3veshoczfl0p0c12cmrvk6l111lur77e08wh6oqozxetp32s8kzvu0" --ptxt "Hello World"
# OR
dc encr ckey "tkcbxp3ws61g7x6y44wcs68a8h71jjh8adbi3yg7q4e9btrdbqujczirog56ojbauewuihc7tb3veshoczfl0p0c12cmrvk6l111lur77e08wh6oqozxetp32s8kzvu0" ptxt "Hello World"
```

- Decrypt a message with a key:

```bash
dc -d -c "tkcbxp3ws61g7x6y44wcs68a8h71jjh8adbi3yg7q4e9btrdbqujczirog56ojbauewuihc7tb3veshoczfl0p0c12cmrvk6l111lur77e08wh6oqozxetp32s8kzvu0" -p "171392.107120.257088.257088.321360
492752.321360.385632.257088.85696"
# OR
dc --decr --ckey "tkcbxp3ws61g7x6y44wcs68a8h71jjh8adbi3yg7q4e9btrdbqujczirog56ojbauewuihc7tb3veshoczfl0p0c12cmrvk6l111lur77e08wh6oqozxetp32s8kzvu0" --ptxt "171392.107120.257088.257088.321360
492752.321360.385632.257088.85696"
# OR
dc decr ckey "tkcbxp3ws61g7x6y44wcs68a8h71jjh8adbi3yg7q4e9btrdbqujczirog56ojbauewuihc7tb3veshoczfl0p0c12cmrvk6l111lur77e08wh6oqozxetp32s8kzvu0" ptxt "171392.107120.257088.257088.321360
492752.321360.385632.257088.85696"
```
### REST API

Make sure the following two environment variables are set: `PORT` and `ACTIX_WEB_ADDR`. To execute and run the REST API binary, simply run it with the `dca` command. Depending on what the environment variables were set to, a local server will be started. The server supports the following API routes:

- Encrypt a message:

```Text
Route: /e/key/message_with_spaces_separated_by_underscores
Example: http://localhost:8000/e/tkcbxp3ws61g7x6y44wcs68a8h71jjh8adbi3yg7q4e9btrdbqujczirog56ojbauewuihc7tb3veshoczfl0p0c12cmrvk6l111lur77e08wh6oqozxetp32s8kzvu0/Hello_World
```

- Decrypt a message:

```Text
Route: /d/key/message_with_newlines_separated_by_pipes
Example: http://localhost:8000/d/tkcbxp3ws61g7x6y44wcs68a8h71jjh8adbi3yg7q4e9btrdbqujczirog56ojbauewuihc7tb3veshoczfl0p0c12cmrvk6l111lur77e08wh6oqozxetp32s8kzvu0/171392.107120.257088.257088.321360|492752.321360.385632.257088.85696
```

- Generate a key with the highest security level:

```Text
Route: /genkey/iron
Example: http://localhost:8000/genkey/iron
```

- Generate a key with mid-level security:

```Text
Route: /genkey/ssec
Example: http://localhost:8000/genkey/ssec
```

- Generate a key with the lowest security level:

```Text
Route: /genkey/msec
Example: http://localhost:8000/genkey/msec
```

The REST API is deployed [here](https://doll-cipher.onrender.com).

## CHANGELOG :black_nib:

### Version 0.1.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- *Doll Cipher :lock: :dolls:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.
