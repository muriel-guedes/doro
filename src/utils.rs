use std::path::Path;

pub fn get_file_language_id(path: &Path) -> String {
    match path.extension().unwrap().to_str().unwrap() {
        "rs" => "rust",
        ext => panic!("Extension: '{ext}', not supported !")
    }.to_string()
}