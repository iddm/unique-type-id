# unique-type-id
A rust procedural macro crate for generating unique id for the rust types.

[![Crates badge](https://img.shields.io/crates/v/unique_type_id.svg)](https://crates.io/crates/unique-type-id) 
[![CI](https://github.com/iddm/unique-type-id/actions/workflows/ci.yml/badge.svg)](https://github.com/iddm/unique-type-id/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/unique-type-id/badge.svg)](https://docs.rs/unique-type-id)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/iddm)](https://github.com/sponsors/iddm)


## What does it do?

It simply implements a trait for the type where is only one method - `id() -> TypeId` which returns a unique positive number. For id generation, the procedural macro reads the file called "types.toml" and searches for the type name
there. You may also specify another file name if you want by using `UniqueTypeIdFile` attribute. Speaking more detailed:

1. The procedural macro reads the attributes on a type.
2. If there are no attributes, it uses `types.toml` file name as types file name, otherwise uses specified one.
3. For each type the macro is used it tries to find the type name in the types file. If it can find it, it returns
it's id, otherwise it returns the available id. Reading tests helps in understanding this.

## Usage

1. Add `unique-type-id` as dependency in your `Cargo.toml`:

```toml
[dependencies]
unique-type-id = "1"
```

2. Create a struct or enum and use the trait:

```rust
#[test]
fn unique_simple() {
    use unique_type_id::UniqueTypeId;
    #[derive(UniqueTypeId)]
    struct Test1;
    #[derive(UniqueTypeId)]
    struct Test2;

    assert_eq!(Test1::id().0, 1u64);
    assert_eq!(Test2::id().0, 2u64);
}
```
 
This will generate a types file if it has not been created yet and put there ids, starting with `0`,
for each type which was not found there. This is how it looks when you have predefined set of ids
for your types:

```rust
#[test]
fn unique_different_file() {
    use unique_type_id::UniqueTypeId;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types2.toml"]
    struct Test1;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types2.toml"]
    struct Test2;

    assert_eq!(Test1::id().0, 115u64);
    assert_eq!(Test2::id().0, 232u64);
}
```

Here we set up ids for our types manually by creating the `types2.toml` file.

## Options

- `UniqueTypeIdFile` - allows to specify the file name to write/read the IDs from.
- `UniqueTypeIdType` - allows to change the ID number type from `u64` (the default) to the
user-preferred one.
- `UniqueTypeIdStart` - allows to set the starting ID number for the type. Can be used if the
type layout file is very well-known and guaranteed to avoid collisions.

### UniqueTypeIdFile

```rust
#[derive(UniqueTypeId)]
#[UniqueTypeIdFile = "types2.toml"]
struct Test1;
```

### UniqueTypeIdType

```rust
#[derive(UniqueTypeId)]
#[UniqueTypeIdType = "i16"]
struct Test;
```

### UniqueTypeIdStart

```rust
#[derive(UniqueTypeId)]
#[UniqueTypeIdStart = "23"]
struct Test;
```

## Note

Default and custom type files are searched relatively to a directory where `cargo build` is called.
 
## License

This project is [licensed under the MIT license](https://github.com/iddm/unique-type-id/blob/master/LICENSE).
