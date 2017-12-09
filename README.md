# unique-type-id
A rust procedural macro crate for generating unique id for the rust types.


[![](https://meritbadge.herokuapp.com/unique-type-id)](https://crates.io/crates/unique-type-id) [![](https://travis-ci.org/vityafx/unique-type-id.svg?branch=master)](https://travis-ci.org/vityafx/unique-type-id)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


## What does it do?

It simply implements a trait for the type where is only one method - `id() -> TypeId` which returns a unique positive number. Unique in the whole project, but not everywhere else.
  

## Usage

1. Add `unique-type-id` as dependency in your `Cargo.toml`:

 ```toml
 [dependencies]
 unique-type-id-derive = "0.1"
 unique-type-id = "0.1"
 ```

2. Create a struct or enum and use the trait:

 ```rust
 #[macro_use]
 extern crate unique_type_id_derive;
 extern crate unique_type_id;

 #[test]
 fn sequential_simple() {
     use unique_type_id::{ SequentialTypeId };
     #[derive(SequentialTypeId)]
     struct Test1;
     #[derive(SequentialTypeId)]
     struct Test2;
     
     assert_eq!(Test1::id().0, 0u64);
     assert_eq!(Test2::id().0, 1u64);
 }
 ```
 
## License

This project is [licensed under the MIT license](https://github.com/vityafx/introspection/blob/master/LICENSE).
