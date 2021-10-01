use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::collections::HashMap;

pub struct Meta {
    file: File,
}

impl Meta {
    pub fn from(path: &String) -> Meta {
        let file_path = format!("{}/meta.txt", path);
        Meta {
            file: OpenOptions::new()
                .read(true)
                .write(true)
                .open(&file_path)
                .unwrap(),
        }
    }

    pub fn exists(path: &String) -> bool {
        let file_path = format!("{}/meta.txt", path);
        Path::new(&file_path).exists()
    }

    pub fn create(path: &String) -> Meta {
        let file_path = format!("{}/meta.txt", path);
        Meta {
            file: OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(&file_path)
                .unwrap(),
        }
    }

    pub fn update(&mut self, key: &String, value: u64) {
        let mut data = self.read();
        data.insert(key.clone(), value);
        self.write(&data);
    }

    fn read(&mut self) -> HashMap<String, u64> {
        let mut data = HashMap::new();
        let mut content = String::new();
        self.file.read_to_string(&mut content).unwrap();

        let mut key = String::new();
        let mut value = String::new();
        let mut target = &mut key;
        for token in content.chars() {
            if token == '=' {
                target = &mut value;
            } else if token == '\n' {
                data.insert(key.clone(), value.clone().parse::<u64>().unwrap());
                key.clear();
                value.clear();
                target = &mut key;
            } else {
                target.push(token);
            }
        }

        data
    }

    fn write(&mut self, data: &HashMap<String, u64>) {
        let mut content = String::new();
        for (key, value) in data.iter() {
            content.push_str(&key);
            content.push('=');
            content.push_str(&value.to_string());
        }
        self.file.set_len(0).unwrap();
        self.file.write(content.as_bytes()).unwrap();
    }
}
