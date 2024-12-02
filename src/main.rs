mod file_reader;
use file_reader::*;

fn main() {
    if !file_exists() {
	let successful_creation = create_file();
	if successful_creation.is_err() {
	    println!("The file could not be created");
	}
    }

    let user_creation = create_new_user("Karl Haidinyak".to_string(), "Cannot Read".to_string());
    if user_creation.is_err() {
	println!("The user couldn't be created");
    }
	
}
