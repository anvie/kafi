extern crate bincode;
extern crate serde;

use std::borrow::Borrow;
use std::cmp::Eq;
use std::collections::HashMap as StdHashMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::result::Result;

use bincode::{deserialize, serialize, Infinite};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

pub struct Store<K, V>
where
    K: Serialize + DeserializeOwned + Eq + Hash,
    V: Serialize + DeserializeOwned + Eq + Hash,
{
    file_path: PathBuf,
    data: StdHashMap<K, V>,
    _modified: bool,
}

impl<K, V> Store<K, V>
where
    K: Serialize + DeserializeOwned + Eq + Hash,
    V: Serialize + DeserializeOwned + Eq + Hash,
{
    pub fn open(path: &str) -> Result<Self, String> {
        let path = Path::new(path);

        let _new = !path.exists();

        let data = if !_new {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open(path)
                .map_err(|e| e.description().to_string())?;

            let mut buf = Vec::new();

            let read = file
                .read_to_end(&mut buf)
                .map_err(|e| e.description().to_string())?;

            let mut _data: StdHashMap<K, V> = StdHashMap::new();

            if read > 0 {
                _data = deserialize(&buf).map_err(|e| e.description().to_string())?;
            }
            _data
        } else {
            StdHashMap::new()
        };

        Ok(Store {
            file_path: path.to_path_buf(),
            data,
            _modified: false,
        })
    }

    pub fn get<Q>(&mut self, key: &Q) -> Option<&V>
    where
        Q: ?Sized + std::hash::Hash + Eq,
        K: Borrow<Q>,
    {
        self.data.get(key.borrow())
    }

    pub fn exists<Q>(&mut self, key: &Q) -> bool
    where
        Q: ?Sized + std::hash::Hash + Eq,
        K: Borrow<Q>,
    {
        self.data.get(key.borrow()).is_some()
    }

    pub fn insert(&mut self, key: K, v: V) {
        self.data.insert(key, v);
        self._modified = true;
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: ?Sized + std::hash::Hash + Eq,
        K: Borrow<Q>,
    {
        let rv = self.data.remove(key);
        self._modified = true;
        rv
    }

    pub fn flush(&mut self) -> Result<(), String> {
        if self._modified {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&self.file_path)
                .map_err(|e| e.description().to_string())?;

            let encoded = serialize(&self.data, Infinite).unwrap();

            let rv = file
                .write_all(&*encoded)
                .map_err(|e| e.description().to_string());

            self._modified = false;

            rv
        } else {
            Ok(())
        }
    }

    pub fn get_path<'a>(&'a self) -> &Path {
        self.file_path.as_path()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl<K, V> Drop for Store<K, V>
where
    K: Serialize + DeserializeOwned + Eq + Hash,
    V: Serialize + DeserializeOwned + Eq + Hash,
{
    fn drop(&mut self) {
        match self.flush() {
            Err(e) => {
                eprintln!("Cannot flush {}: {:?}", self.file_path.display(), e);
                panic!(format!(
                    "Cannot flush {}: {:?}",
                    self.file_path.display(),
                    e
                ));
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {

    extern crate rand;

    use super::*;
    use std::env;
    use std::fs::remove_file;
    use test::rand::Rng;

    fn _gen_filename() -> String {
        let mut dir = env::temp_dir();
        let file_name: String = rand::thread_rng().gen_ascii_chars().take(12).collect();
        dir.push(file_name);
        dir.set_extension("kafidb");
        println!("{:?}", dir.as_path().display());
        dir.into_os_string().into_string().unwrap()
    }

    fn _gen_col() -> Store<String, String> {
        let mut col = Store::open(&_gen_filename()).unwrap();
        col.insert("satu".to_string(), "111".to_string());
        let _ = col.flush();
        col
    }

    #[test]
    fn test_exists() {
        let mut col = _gen_col();
        assert_eq!(col.exists("satu"), true);
        assert_eq!(col.exists("dua"), false);
        remove_file(col.get_path()).unwrap();
    }

    #[test]
    fn test_get() {
        let mut col = _gen_col();
        assert_eq!(col.get("satu"), Some(&"111".to_string()));
        assert_eq!(col.get("lima"), None);
        remove_file(col.get_path()).unwrap();
    }

    #[test]
    fn test_remove() {
        let mut col = _gen_col();
        assert_eq!(col.remove("satu"), Some("111".to_string()));
        assert_eq!(col.exists("satu"), false);
        remove_file(col.get_path()).unwrap();
    }

    #[test]
    fn test_clear() {
        let mut col = _gen_col();
        col.clear();
        assert_eq!(col.exists("satu"), false);
        remove_file(col.get_path()).unwrap();
    }

    #[test]
    fn test_already_filled() {
        let path = _gen_filename();
        {
            use std::fs;
            let _ = fs::remove_file(&path);
        }
        {
            let mut col: Store<String, String> = Store::open(&path).unwrap();
            col.insert("satu".to_string(), "111".to_string());
            col.flush().unwrap();
        }
        {
            let mut col: Store<String, String> = Store::open(&path).unwrap();
            assert_eq!(col.exists("satu"), true);
            assert_eq!(col.get("satu"), Some(&"111".to_string()));
            assert_eq!(col.get("lima"), None);
            col.insert("lima".to_string(), "555".to_string());
            assert_eq!(col.get("lima"), Some(&"555".to_string()));
            col.flush().unwrap();
        }
        remove_file(path).unwrap();
    }

    #[test]
    fn test_auto_flush() {
        let path = _gen_filename();
        {
            use std::fs;
            let _ = fs::remove_file(&path);
        }
        {
            let mut col: Store<String, String> = Store::open(&path).unwrap();
            col.insert("satu".to_string(), "111".to_string());
            assert_eq!(std::path::Path::new(&path).exists(), false);
        }
        assert_eq!(std::path::Path::new(&path).exists(), true);
        remove_file(path).unwrap();
    }
}
