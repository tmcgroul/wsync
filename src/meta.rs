use std::fs::{File, write, read_to_string};
use std::path::Path;
use std::collections::HashMap;

pub struct Meta {
    path: String,
}

impl Meta {
    pub fn from(folder: &String) -> Meta {
        Meta {
            path: format!("{}/meta.txt", folder),
        }
    }

    pub fn exists(folder: &String) -> bool {
        let file_path = format!("{}/meta.txt", folder);
        Path::new(&file_path).exists()
    }

    pub fn create(folder: &String) -> Meta {
        let file_path = format!("{}/meta.txt", folder);
        File::create(&file_path).unwrap();
        Meta {
            path: file_path,
        }
    }

    pub fn update(&self, key: &String, value: u64) {
        let mut data = self.read();
        data.insert(key.clone(), value);
        self.write(&data);
    }

    fn read(&self) -> HashMap<String, u64> {
        let mut data = HashMap::new();
        let content = read_to_string(&self.path).unwrap();

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

    fn write(&self, data: &HashMap<String, u64>) {
        let mut content = String::new();
        for (key, value) in data.iter() {
            content.push_str(&key);
            content.push('=');
            content.push_str(&value.to_string());
        }
        write(&self.path, content).unwrap();
    }
}
