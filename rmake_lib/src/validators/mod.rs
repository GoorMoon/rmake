use std::path::Path;

pub fn path_is_exists(path: String) -> Result<(), String> {
    if Path::new(&path).is_file() {
        Ok(())
    } else {
        Err(format!("The file <{}> doesn't exists", path))
    }
}
