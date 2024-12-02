use std::io::BufReader;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Error;
use std::io::Write;
use sha2::{Sha256, Digest};
use std::path::Path;

pub fn file_exists() -> bool {
    let path = Path::new("./users.csv");
    return path.exists();
}

pub fn create_file() -> Result<(), Error> {
    if file_exists() {
	return Err(io::Error::new(io::ErrorKind::AlreadyExists, "The file already exists"));
    }
    
    let mut new_file = File::create("users.csv")?;
    let bytes = "Username, Password\n".as_bytes();
    new_file.write_all(&bytes)?;

    return Ok(());
}

pub fn create_new_user(username: String, password: String) -> Result<(), Error> {
    let mut users = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./users.csv")?;
    
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();

    let password_str = base64::encode(result);
    
    let user_data = format!("{}, {}\n", username, password_str);
    users.write_all(user_data.as_bytes())?;
    
    return Ok(());
}

pub fn read_users() -> std::io::Result<Vec<(String, String)>> {
    let mut users: Vec<(String, String)> = Vec::new();
    let users_file = File::open("users.csv")?;
    let buf_reader = BufReader::new(users_file);
    

    for line in buf_reader.lines() {
	let line = line?;
	let parts = line.split(",");
	let collection: Vec<&str> = parts.collect();

	if collection.len() != 2 {
	    continue;
	}
	
	let username: String = collection[0].to_string();
	let password: String = collection[0].to_string();

	users.push((username, password));
    }

    return Ok(users); 
}
