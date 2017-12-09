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

#[test]
fn sequential_simple_second() {
    use unique_type_id::{ SequentialTypeId };
    #[derive(SequentialTypeId)]
    struct Test1;
    #[derive(SequentialTypeId)]
    struct Test2;

    assert_eq!(Test1::id().0, 1u64);
    assert_eq!(Test2::id().0, 2u64);
}

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
