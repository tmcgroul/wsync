use std::collections::HashMap;
use std::fs::{read_to_string, write, File};
use std::io;
use std::num::ParseIntError;
use std::path::Path;

#[derive(Debug)]
pub enum ReadError {
    IoError(io::Error),
    ParseError(ParseIntError),
}

impl From<io::Error> for ReadError {
    fn from(error: io::Error) -> Self {
        ReadError::IoError(error)
    }
}

impl From<ParseIntError> for ReadError {
    fn from(error: ParseIntError) -> Self {
        ReadError::ParseError(error)
    }
}

pub struct Meta {
    path: String,
}

impl Meta {
    pub fn new(folder: &str) -> Meta {
        Meta {
            path: format!("{}/meta.txt", folder),
        }
    }

    pub fn exists(folder: &str) -> bool {
        let file_path = format!("{}/meta.txt", folder);
        Path::new(&file_path).exists()
    }

    pub fn create(folder: &str) -> io::Result<Meta> {
        let file_path = format!("{}/meta.txt", folder);
        File::create(&file_path)?;
        Ok(Meta { path: file_path })
    }

    pub fn get(&self, key: &str) -> Result<Option<u64>, ReadError> {
        let data = self.read()?;
        Ok(data.get(key).copied())
    }

    pub fn update(&self, key: &str, value: u64) -> Result<(), ReadError> {
        let mut data = self.read()?;
        data.insert(key.to_string(), value);
        self.write(&data)?;
        Ok(())
    }

    fn read(&self) -> Result<HashMap<String, u64>, ReadError> {
        let mut data = HashMap::new();
        let content = read_to_string(&self.path)?;

        let mut key = String::new();
        let mut value = String::new();
        let mut target = &mut key;
        for line in content.split('\n') {
            if line.is_empty() {
                continue;
            }

            for token in line.chars() {
                if token == '=' {
                    target = &mut value;
                } else {
                    target.push(token);
                }
            }
            data.insert(key.clone(), value.clone().parse::<u64>()?);
            key.clear();
            value.clear();
            target = &mut key;
        }

        Ok(data)
    }

    fn write(&self, data: &HashMap<String, u64>) -> io::Result<()> {
        let mut content = String::new();
        for (key, value) in data.iter() {
            content.push_str(key);
            content.push('=');
            content.push_str(&value.to_string());
        }
        write(&self.path, content)?;
        Ok(())
    }
}
