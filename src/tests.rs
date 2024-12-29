#[cfg(test)]
mod tests {
    use crate::file_reader::*;
    use std::io::{Error, ErrorKind};
    use std::path::Path;

    #[test]
    fn create_file_tests() {
        let result: Result<bool, Error> = create_file();
        let path = Path::new("./src/users.csv");

        if let Err(err) = result {
            assert_eq!(err.kind(), ErrorKind::AlreadyExists);
            assert!(path.exists());
        } else {
            panic!("Expected an error, but got Ok!");
        }
    }
}
