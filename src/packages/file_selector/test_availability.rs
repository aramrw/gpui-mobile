
#[cfg(test)]
mod tests {
    use crate::packages::file_selector::{OpenFileOptions, open_file};

    #[test]
    fn test_file_selector_availability() {
        let options = OpenFileOptions {
            accept_type_groups: vec![],
            initial_directory: None,
        };

        // If we are on a platform that should support it, this should return
        // a result other than the "not available" error.
        let result = futures::executor::block_on(open_file(options));
        
        match result {
            Err(e) if e == "File selector is not available on this platform or feature is disabled" => {
                panic!("File selector reported as unavailable: {}", e);
            }
            _ => {
                println!("File selector result: {:?}", result);
            }
        }
    }
}
