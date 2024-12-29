use crate::file_reader::*;
use std::io::{Error, ErrorKind};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_file_tests() {
        let result: Result<bool, Error> = create_file();
        if let Err(err) = result {
            assert_eq!(err.kind(), ErrorKind::AlreadyExists);
        } else {
            panic!("Expected an error, but got Ok!");
        }
    }
}
