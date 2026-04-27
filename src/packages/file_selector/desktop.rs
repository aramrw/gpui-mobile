//! Desktop implementation of the file selector using `rfd`.
use crate::packages::file_selector::{OpenFileOptions, SaveFileOptions, SelectedFile};

#[cfg(all(not(any(target_os = "ios", target_os = "android")), feature = "file_selector"))]
use rfd::FileDialog;

#[cfg(all(not(any(target_os = "ios", target_os = "android")), feature = "file_selector"))]
fn map_options(options: OpenFileOptions) -> FileDialog {
    let mut dialog = FileDialog::new();
    for group in options.accept_type_groups {
        dialog = dialog.add_filter(
            group.label,
            &group
                .extensions
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>(),
        );
    }
    if let Some(dir) = options.initial_directory {
        dialog = dialog.set_directory(dir);
    }
    dialog
}

pub async fn open_file(options: OpenFileOptions) -> Result<Option<SelectedFile>, String> {
    #[cfg(all(not(any(target_os = "ios", target_os = "android")), feature = "file_selector"))]
    {
        let dialog = map_options(options);
        let file = dialog.pick_file();
        Ok(file.map(|f| SelectedFile {
            path: f.to_string_lossy().to_string(),
            name: f
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default(),
        }))
    }
    #[cfg(not(all(not(any(target_os = "ios", target_os = "android")), feature = "file_selector")))]
    {
        let _ = options;
        Err("File selector is not available on this platform or feature is disabled".into())
    }
}

pub async fn open_files(options: OpenFileOptions) -> Result<Vec<SelectedFile>, String> {
    #[cfg(all(not(any(target_os = "ios", target_os = "android")), feature = "file_selector"))]
    {
        let dialog = map_options(options);
        let files = dialog.pick_files();
        Ok(files
            .unwrap_or_default()
            .iter()
            .map(|f| SelectedFile {
                path: f.to_string_lossy().to_string(),
                name: f
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default(),
            })
            .collect())
    }
    #[cfg(not(all(not(any(target_os = "ios", target_os = "android")), feature = "file_selector")))]
    {
        let _ = options;
        Err("File selector is not available on this platform or feature is disabled".into())
    }
}

pub fn get_save_path(options: &SaveFileOptions) -> Result<Option<String>, String> {
    #[cfg(all(not(any(target_os = "ios", target_os = "android")), feature = "file_selector"))]
    {
        let mut dialog = FileDialog::new();
        for group in &options.accept_type_groups {
            dialog = dialog.add_filter(
                &group.label,
                &group
                    .extensions
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>(),
            );
        }
        if let Some(dir) = &options.initial_directory {
            dialog = dialog.set_directory(dir);
        }
        if let Some(name) = &options.suggested_name {
            dialog = dialog.set_file_name(name);
        }
        let path = dialog.save_file();
        Ok(path.map(|p| p.to_string_lossy().to_string()))
    }
    #[cfg(not(all(not(any(target_os = "ios", target_os = "android")), feature = "file_selector")))]
    {
        let _ = options;
        Err("File selector is not available on this platform or feature is disabled".into())
    }
}

pub fn get_directory_path(initial_directory: Option<&str>) -> Result<Option<String>, String> {
    #[cfg(all(not(any(target_os = "ios", target_os = "android")), feature = "file_selector"))]
    {
        let mut dialog = FileDialog::new();
        if let Some(dir) = initial_directory {
            dialog = dialog.set_directory(dir);
        }
        let path = dialog.pick_folder();
        Ok(path.map(|p| p.to_string_lossy().to_string()))
    }
    #[cfg(not(all(not(any(target_os = "ios", target_os = "android")), feature = "file_selector")))]
    {
        let _ = initial_directory;
        Err("File selector is not available on this platform or feature is disabled".into())
    }
}
