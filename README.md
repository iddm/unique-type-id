# unique-type-id
A rust procedural macro crate for generating unique id for the rust types.


[![](https://meritbadge.herokuapp.com/unique-type-id)](https://crates.io/crates/unique-type-id) [![](https://travis-ci.org/vityafx/unique-type-id.svg?branch=master)](https://travis-ci.org/vityafx/unique-type-id)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


## What does it do?

It simply implements a trait for the type where is only one method - `id() -> TypeId` which returns a unique positive number. For id generation, the procedural macro reads the file called "types.toml" and searches for the type name
there. You may also specify another file name if you want by using `UniqueTypeIdFile` attribute. Speaking more detailed:

1. The procedural macro reads the attributes on a type.
2. If there are no attributes, it uses `types.toml` file name as types file name, otherwise uses specified one.
3. For each type the macro is used it tries to find the type name in the types file. If it can find it, it returns
it's id, otherwise it returns the (maximum-id + 1). Reading tests helps in understanding of this.
  

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
     
     assert_eq!(Test1::id().0, 1u64);
     assert_eq!(Test2::id().0, 2u64);
 }
 ```
 
 This will generate a types file if it has not been created yet and put there ids, starting with 1, for each type
 which was not found there. This is how it looks when you have predefined set of ids for your types:
 
```rust 
#[test]
fn sequential_different_file() {
    use unique_type_id::{ SequentialTypeId };
    #[derive(SequentialTypeId)]
    #[UniqueTypeIdFile = "types2.toml"]
    struct Test1;
    #[derive(SequentialTypeId)]
    #[UniqueTypeIdFile = "types2.toml"]
    struct Test2;

    assert_eq!(Test1::id().0, 115u64);
    assert_eq!(Test2::id().0, 232u64);
}
```

Here we set up ids for our types manually by creating the `types2.toml` file.

## Note

Default and custom type files are searched relatively to a directory where `cargo build` is called.
 
## License

This project is [licensed under the MIT license](https://github.com/vityafx/introspection/blob/master/LICENSE).
