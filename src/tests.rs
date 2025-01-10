#[cfg(test)]
mod tests {
    use crate::file_reader::*;
    use std::io::{Error, ErrorKind};
    use std::path::Path;

    #[test]
    fn create_new_user_tests() {
        let username = String::from("Karl Haidinyak");
        let result = contains_username(&username);

        match result {
            Ok(is_contained) => {
                assert!(is_contained);
            } Err(_) => {
                panic!();
            }
        }

        let username = String::from("karl haidinyak");
        let result = contains_username(&username);

        match result {
            Ok(is_contained) => {
                assert!(!is_contained);
            } Err(_) => {
                panic!();
            }
        }
    }

    #[test]
    fn valid_user_input_tests() {
        let new_username = String::from("Karl");
        let new_password = String::from("haidinyak");

        match create_new_user(&new_username, &new_password) {
            _ => {
                let result = valid_user_input(&new_username, &new_password);

                match result {
                    Ok(is_valid) => {
                        assert!(is_valid);
                    } Err(_) => {
                        panic!();
                    }
                }
            }
        }

        let fake_username = String::from("fake");
        let fake_password = String::from("fake");

        let result = valid_user_input(&fake_username, &fake_password);
        match result {
            Ok(_) => {
                panic!();
            } Err(err) => {
                assert_eq!(ErrorKind::InvalidData, err.kind());
            }
        }

    }

    #[test]
    fn trie_insert() {
        
    }
}
