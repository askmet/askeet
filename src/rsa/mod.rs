#![allow(unused)]

use colored::Colorize;
use inquire::{
    max_length, min_length, required,
    ui::{Attributes, StyleSheet, Styled},
    validator::Validation,
    CustomUserError, InquireError, Password, Text,
};
use openssl::rsa::Rsa;

use std::{
    env,
    error::Error,
    fmt::format,
    fs::File,
    io::{stdin, stdout, Write},
    path::{Path, PathBuf},
};

use crate::cli::inquire::get_render_config;

pub fn generate_private_key(bits: &usize) {
    let render_config = get_render_config();

    let passphrase = Password::new(&"Enter the passphrase:")
        .with_validator(required!("Passphrase is required"))
        .with_render_config(render_config)
        .with_display_toggle_enabled()
        .prompt();

    let passphrase = match passphrase {
        Ok(pass) => pass,
        Err(err) => {
            println!("{}", err.to_string());
            panic!()
        }
    };

    let pair_name = Text::new(&"Enter the name of pair:")
        .with_render_config(render_config)
        .with_validator(required!("This field is required"))
        .with_validator(validation_name_key_pair)
        .with_validator(max_length!(30))
        .with_validator(min_length!(5))
        .prompt();

    let pair_name = match pair_name {
        Ok(pair_name) => pair_name,
        Err(err) => {
            println!("{}", err.to_string());
            panic!();
        }
    };

    let passphrase = passphrase.as_bytes();
    let rsa = Rsa::generate(*bits as u32).unwrap();
    let cipher = openssl::symm::Cipher::aes_256_cbc();

    let private_key = rsa
        .private_key_to_pem_passphrase(cipher, passphrase)
        .unwrap();

    let public_key = rsa.public_key_to_pem().unwrap();

    match save_pair_to_file(
        &pair_name,
        &String::from_utf8(public_key).unwrap(),
        &String::from_utf8(private_key).unwrap(),
    ) {
        Ok(save_result) => {
            println!(
                "{}: {}\n{}: {}",
                "Public Key".bold(),
                save_result.path_public_file.as_os_str().to_string_lossy(),
                "Private Key".bold(),
                save_result.path_private_file.as_os_str().to_string_lossy()
            );
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

struct SaveResult {
    path_public_file: PathBuf,
    path_private_file: PathBuf,
}

fn get_directory_for_save() -> PathBuf {
    #[cfg(debug_assertions)]
    return env::current_dir().unwrap().join("keys");

    return env::current_dir().unwrap();
}

fn save_pair_to_file(
    pair_name: &String,
    public_key: &String,
    private_key: &String,
) -> Result<SaveResult, std::io::Error> {
    let public_key_file_name = format!("{}.public.pem", pair_name);
    let private_key_file_name = format!("{}.private.pem", pair_name);

    let mut public_key_file = File::create(get_directory_for_save().join(public_key_file_name))?;

    let mut private_key_file = File::create(get_directory_for_save().join(private_key_file_name))?;

    public_key_file.write_all(public_key.as_bytes())?;
    private_key_file.write_all(private_key.as_bytes())?;

    Ok(SaveResult {
        path_private_file: get_directory_for_save().join(format!("{}.public.pem", pair_name)),
        path_public_file: get_directory_for_save().join(format!("{}.private.pem", pair_name)),
    })
}

fn validation_name_key_pair(text: &str) -> Result<Validation, CustomUserError> {
    let have_incorrect_symbol = text.chars().any((|symbol| !symbol.is_alphabetic()));

    Ok(if have_incorrect_symbol {
        Validation::Invalid("Have incorrect symbols".into())
    } else {
        Validation::Valid
    })
}
