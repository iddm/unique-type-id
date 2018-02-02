extern crate unique_type_id;
#[macro_use]
extern crate unique_type_id_derive;

#[test]
fn check_simple() {
    use unique_type_id::UniqueTypeId;
    #[derive(UniqueTypeId)]
    struct Test1;
    #[derive(UniqueTypeId)]
    struct Test2;

    assert_eq!(Test1::id().0, 1u64);
    assert_eq!(Test2::id().0, 2u64);
}

#[test]
fn check_simple_second() {
    use unique_type_id::UniqueTypeId;
    #[derive(UniqueTypeId)]
    struct Test1;
    #[derive(UniqueTypeId)]
    struct Test2;
    #[derive(UniqueTypeId)]
    struct Test3;
    #[derive(UniqueTypeId)]
    struct Test4;
    #[derive(UniqueTypeId)]
    struct Test5;

    assert_eq!(Test1::id().0, 1u64);
    assert_eq!(Test2::id().0, 2u64);
    assert_eq!(Test3::id().0, 5u64);
    assert_eq!(Test4::id().0, 0u64);
    assert_eq!(Test5::id().0, 3u64);
}

#[test]
fn check_different_file() {
    use unique_type_id::UniqueTypeId;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types2.toml"]
    struct Test1;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types2.toml"]
    struct Test2;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types2.toml"]
    struct Test3;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types2.toml"]
    struct Test4;

    assert_eq!(Test1::id().0, 115u64);
    assert_eq!(Test2::id().0, 232u64);
    assert_eq!(Test3::id().0, 0u64);
    assert_eq!(Test4::id().0, 1u64);
}

#[test]
fn check_simple_empty_file() {
    use unique_type_id::UniqueTypeId;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types3.toml"]
    struct Test1;
    #[derive(UniqueTypeId)]
    #[UniqueTypeIdFile = "types3.toml"]
    struct Test2;

    assert_eq!(Test1::id().0, 0u64);
    assert_eq!(Test2::id().0, 1u64);
}
