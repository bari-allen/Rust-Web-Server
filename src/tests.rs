#[cfg(test)]
mod file_reader_tests {
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
}

#[cfg(test)]
mod trie_tests {
    use crate::trie::Trie;
    #[test]
    fn trie_contains_tests() {
        let mut trie = Trie::new();
        trie.insert("Hello");
        trie.insert("He");
        trie.insert("Welcome Back My Friends");
        trie.insert("Welcome To My Humble Abode");

        let contains = trie.contains("Hello");
        let not_contains = trie.contains("hello");
        assert!(contains && !not_contains);

        let contains = trie.contains("Welcome Back My Friends");
        let not_contains = trie.contains("Welcome Back My FRiends");
        assert!(contains && !not_contains);

        let contains = trie.contains("He");
        let not_contains = trie.contains("HE");
        assert!(contains && !not_contains);

        let contains = trie.contains("Welcome To My Humble Abode");
        let not_contains = trie.contains("Welcome");
        assert!(contains && !not_contains);
    }

    #[test]
    fn trie_suggest_tests() {
        let mut trie = Trie::new();
        trie.insert("Hello");
        trie.insert("Help");
        trie.insert("Healing");

        let mut suggestions = trie.suggest("He");
        suggestions.sort();
        let actual = vec!["Healing", "Hello", "Help"];
        assert_eq!(suggestions, actual);

        trie.insert("Welcome Back My Friends");
        trie.insert("Well Well Well");
        trie.insert("We Can Do This");

        let mut suggestions = trie.suggest("We");
        suggestions.sort();
        let actual = vec!["We Can Do This",
                "Welcome Back My Friends", "Well Well Well"];
        assert_eq!(actual, suggestions);
    }
}
