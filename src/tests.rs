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
}
