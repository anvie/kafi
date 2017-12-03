KAFI
=======

Persistent key value store library for Rust.


Usage
------

```rust

extern crate kafi;

use kafi::Store;

let mut store:Store<String, String> = Store::open("kafi.db").unwrap();

store.insert("satu".to_string(), "111".to_string());
assert_eq!(store.exists("satu"), true);

store.flush().unwrap(); // <-- call flush to persist into disk

assert_eq!(store.get("satu"), Some(&"111".to_string()));
assert_eq!(store.get("lima"), None);
```


[] Robin Sy.
