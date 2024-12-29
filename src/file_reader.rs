use std::io::{BufReader, Error, Write, ErrorKind};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use sha2::{Sha256, Digest};
use std::path::Path;

///Sees whether the user.csv file exists
///in the current directory
pub fn file_exists() -> bool {
    let path = Path::new("src/users.csv");
    return path.exists();
}

///Checks whether the user's inputted username and password match
///a corresponding username and password in the database
pub fn valid_user_input(username: String, password: String) -> Result<bool, Error> {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let password_str = base64::encode(result);

    let users: Vec<(String, String)> = read_users()?;

    let is_valid:bool = users.into_iter()
        .any(|(database_username, database_password)| username == database_username && password_str == database_password);

    return if is_valid {Ok(true)} else {Err(Error::new(ErrorKind::InvalidData, "Username or Password is invalid"))};
}


///Creates a new file called user.csv
///in the current directory and initializes the
///first line with "Username, Password"
pub fn create_file() -> Result<bool, Error> {
    if file_exists() {
	    return Err(Error::new(ErrorKind::AlreadyExists, "The file already exists"));
    }
    
    let mut new_file = File::create("users.csv")?;
    let bytes = "Username, Password\n".as_bytes();
    new_file.write_all(&bytes)?;

    return Ok(true);
}

fn contains_username(username: String) -> Result<bool, Error> {
    let active_users = read_users();
    match active_users {
        Ok(active_users) => {
            let contains: bool = active_users.into_iter().any(|(active_username, _)| active_username == username);
            return Ok(contains);
        }
        Err(_) => {
            return Err(Error::new(ErrorKind::NotFound, "csv file not found"));
        }
    }
}

///Writes a new user to the user.csv file, if it exists.
///This function hashes the `Password` its given to add
///an extra layer of safety
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
    let users_file = File::open("src/users.csv")?;
    let buf_reader = BufReader::new(users_file);
    

    for line in buf_reader.lines() {
	let line = line?;
	let parts = line.split(",");
	let collection: Vec<&str> = parts.collect();

	if collection.len() != 2 {
	    continue;
	}
	
	let username: String = collection[0].to_string();
	let password: String = collection[1].to_string();

	users.push((username, password));
    }

    return Ok(users);
}
