KAFI
=======

[![Build Status](https://travis-ci.org/anvie/kafi.svg?branch=master)](https://travis-ci.org/anvie/kafi)
[![Build status](https://ci.appveyor.com/api/projects/status/yoqd9gnvb9cv0a94?svg=true)](https://ci.appveyor.com/project/anvie/kafi)
[![Crates.io](https://img.shields.io/crates/v/kafi.svg)](https://crates.io/crates/kafi)

Super simple persistent key value store library for Rust.

Install
--------

```
[dependencies]
kafi = "0.1.2"
```


Usage
------

```rust

extern crate kafi;

use kafi::Store;

let mut store:Store<String, String> = Store::open("kafi.db").unwrap();

store.insert("satu", "111".to_string());
assert_eq!(store.exists("satu"), true);

store.flush().unwrap(); // <-- call flush to persist into disk

assert_eq!(store.get("satu"), Some(&"111".to_string()));
assert_eq!(store.get("lima"), None);
```


[] Robin Sy.
