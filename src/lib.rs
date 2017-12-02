
extern crate serde;
extern crate bincode;

use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::hash::Hash;
use std::cmp::Eq;
use std::path::{Path, PathBuf};
use std::result::Result;
use std::error::Error;
use std::borrow::Borrow;
use std::collections::HashMap as StdHashMap;

use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use bincode::{serialize, deserialize, Infinite};

pub struct HashMap<K, V>
    where K: Serialize + DeserializeOwned + Eq + Hash,
          V: Serialize + DeserializeOwned + Eq + Hash {
    file_path: PathBuf,
    data: StdHashMap<K, V>
}

impl<K, V> HashMap<K, V>
    where K: Serialize + DeserializeOwned + Eq + Hash,
          V: Serialize + DeserializeOwned + Eq + Hash {

    pub fn open(path: &str) -> Result<Self, String> {
        let path = Path::new(path);

        let _new = !path.exists();

        let data = if !_new {

            let mut file =
                OpenOptions::new().read(true).write(true).create(true).append(true)
                    .open(path)
                    .map_err(|e| e.description().to_string())?;

            let mut buf = Vec::new();

            let read = file.read_to_end(&mut buf)
                .map_err(|e| e.description().to_string())?;

            let mut _data:StdHashMap<K, V> = StdHashMap::new();

            if read > 0 {
                _data = deserialize(&buf)
                    .map_err(|e| e.description().to_string())?;
            }
            _data
        }else{
            StdHashMap::new()
        };

        Ok(HashMap {
            file_path: path.to_path_buf(),
            data
        })
    }

    pub fn get<Q>(&mut self, key: &Q) -> Option<&V>
        where Q: ?Sized + std::hash::Hash + Eq, K: Borrow<Q> {
        self.data.get(key.borrow())
    }

    pub fn exists<Q>(&mut self, key: &Q) -> bool
        where Q: ?Sized + std::hash::Hash + Eq, K: Borrow<Q> {
        self.data.get(key.borrow()).is_some()
    }

    pub fn insert(&mut self, key: K, v: V) {
        self.data.insert(key, v);
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
        where Q: ?Sized + std::hash::Hash + Eq, K: Borrow<Q> {
        self.data.remove(key)
    }

    pub fn flush(&mut self) -> Result<(), String> {

        let mut file =
            OpenOptions::new().write(true).create(true).truncate(true)
                .open(&self.file_path)
                .map_err(|e| e.description().to_string())?;

        let encoded = serialize(&self.data, Infinite).unwrap();

        file.write_all(&*encoded)
            .map_err(|e| e.description().to_string())
    }
}

impl<K, V> Drop for HashMap<K, V>
    where K: Serialize + DeserializeOwned + Eq + Hash,
          V: Serialize + DeserializeOwned + Eq + Hash {

    fn drop(&mut self) {
        self.flush().unwrap()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn _gen_col() -> HashMap<String, String> {
        let mut col = HashMap::open("/tmp/_kafi_test_.db").unwrap();
        col.insert("satu".to_string(), "111".to_string());
        let _ = col.flush();
        col
    }

    #[test]
    fn test_exists(){
        let mut col = _gen_col();
        assert_eq!(col.exists("satu"), true);
        assert_eq!(col.exists("dua"), false);
    }

    #[test]
    fn test_get(){
        let mut col = _gen_col();
        assert_eq!(col.get("satu"), Some(&"111".to_string()));
        assert_eq!(col.get("lima"), None);
    }

    #[test]
    fn test_remove(){
        let mut col = _gen_col();
        assert_eq!(col.remove("satu"), Some("111".to_string()));
        assert_eq!(col.exists("satu"), false);
    }

    #[test]
    fn test_already_filled(){
        let path = "/tmp/_kafi_test_2.db";
        {
            use std::fs;
            let _ = fs::remove_file(path);
        }
        {
            let mut col:HashMap<String, String> = HashMap::open(path).unwrap();
            col.insert("satu".to_string(), "111".to_string());
            col.flush().unwrap();
        }
        {
            let mut col:HashMap<String, String> = HashMap::open(path).unwrap();
            assert_eq!(col.exists("satu"), true);
            assert_eq!(col.get("satu"), Some(&"111".to_string()));
            assert_eq!(col.get("lima"), None);
            col.insert("lima".to_string(), "555".to_string());
            assert_eq!(col.get("lima"), Some(&"555".to_string()));
            col.flush().unwrap();
        }
    }
    
    #[test]
    fn test_auto_flush(){
        let path = "/tmp/_kafi_test_3.db";
        {
            use std::fs;
            let _ = fs::remove_file(path);
        }
        {
            let mut col:HashMap<String, String> = HashMap::open(path).unwrap();
            col.insert("satu".to_string(), "111".to_string());
            assert_eq!(std::path::Path::new(path).exists(), false);
        }
        assert_eq!(std::path::Path::new(path).exists(), true);
    }
}

