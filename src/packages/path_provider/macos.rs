use std::path::PathBuf;

pub fn temporary_directory() -> Result<PathBuf, String> {
    Ok(std::env::temp_dir())
}

pub fn documents_directory() -> Result<PathBuf, String> {
    dirs::document_dir().ok_or_else(|| "Could not find documents directory".to_string())
}

pub fn cache_directory() -> Result<PathBuf, String> {
    dirs::cache_dir().ok_or_else(|| "Could not find cache directory".to_string())
}

pub fn support_directory() -> Result<PathBuf, String> {
    dirs::data_dir().ok_or_else(|| "Could not find application support directory".to_string())
}
