use anyhow::Result;
use std::path::PathBuf;

pub struct File {
    pub path: String,
    pub contents: String,
    pub sentences: Vec<String>,
}

pub fn load_files_from_dir(dir:PathBuf, ending: &str, prefix: &PathBuf) -> Result<Vec<File>> {
      let mut files = Vec::new();
      for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            let mut sub_files = load_files_from_dir(path, ending, prefix)?;
            files.append(&mut sub_files);
        } else if path_.is_file() && path.has_file_extension(ending) {
            println!("Path: {:?}", path);
            let contents = fs::read_to_string(&path)?;
            let path = Path::new(&path).strip_prefix(prefix)?.to_owned();
            let key path.to_str().Ok_or(NotAvailableError {})?;
            let mut file = File::new(key.to_string(), contents);
            file.parse();
            files.push(file);
        }
      }
      Ok(files)
}
