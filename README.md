KAFI
=======

Persistent key value store library for Rust.


Usage
------

```rust

extern crate kafi;

use kafi::HashMap;

let mut col:HashMap<String, String> = HashMap::open("kafi.db").unwrap();
col.insert("satu".to_string(), "111".to_string());
assert_eq!(col.exists("satu"), true);

col.flush().unwrap(); // <-- call flush to persist into disk

assert_eq!(col.get("satu"), Some(&"111".to_string()));
assert_eq!(col.get("lima"), None);
```


[] Robin Sy.


