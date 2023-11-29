#![recursion_limit = "128"]

use fs2::FileExt;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use std::fs::File;
use syn::parse_macro_input;

static DEFAULT_TYPES_FILE_NAME: &str = "types.toml";
static DEFAULT_ID_TYPE: &str = "u64";
static DEFAULT_ID_START: &str = "0";

type PairsMap = BTreeMap<String, u64>;

/// `UniqueTypeId`
///
/// ## What does it do?
///
/// It simply implements a trait for the type where is only one method - `id() -> TypeId` which
/// returns a unique positive number. For id generation, the procedural macro reads the file called
/// "types.toml" and searches for the type name there. You may also specify another file name if
/// you want by using `UniqueTypeIdFile` attribute. Speaking more detailed:
///
/// 1. The procedural macro reads the attributes on a type.
/// 2. If there are no attributes, it uses `types.toml` file name as types file name, otherwise
/// uses specified one.
/// 3. For each type the macro is used it tries to find the type name in the types file. If it can
/// find it, it returns it's id, otherwise it returns the available id. Reading tests helps in
/// understanding this.
///
/// ## Usage
///
/// 1. Add `unique-type-id` as dependency in your `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// unique-type-id = "1"
/// ```
///
/// 2. Create a struct or enum and use the trait:
///
/// ```rust,ignore
/// #[test]
/// fn unique_simple() {
///     use unique_type_id::UniqueTypeId;
///     #[derive(UniqueTypeId)]
///     struct Test1;
///     #[derive(UniqueTypeId)]
///     struct Test2;
///
///     assert_eq!(Test1::id().0, 1u64);
///     assert_eq!(Test2::id().0, 2u64);
/// }
/// ```
///
/// This will generate a types file if it has not been created yet and put there ids, starting with
/// `0`, for each type which was not found there. This is how it looks when you have predefined set
/// of ids for your types:
///
/// ```rust,ignore
/// #[test]
/// fn unique_different_file() {
///     use unique_type_id::UniqueTypeId;
///     #[derive(UniqueTypeId)]
///     #[UniqueTypeIdFile = "types2.toml"]
///     struct Test1;
///     #[derive(UniqueTypeId)]
///     #[UniqueTypeIdFile = "types2.toml"]
///     struct Test2;
///
///     assert_eq!(Test1::id().0, 115u64);
///     assert_eq!(Test2::id().0, 232u64);
/// }
/// ```
///
/// Here we set up ids for our types manually by creating the `types2.toml` file.
///
/// ## Options
///
/// - `UniqueTypeIdFile` - allows to specify the file name to write/read the IDs from.
/// - `UniqueTypeIdType` - allows to change the ID number type from `u64` (the default) to the
/// user-preferred one.
/// - `UniqueTypeIdStart` - allows to set the starting ID number for the type. Can be used if the
/// type layout file is very well-known and guaranteed to avoid collisions.
///
/// ### UniqueTypeIdFile
///
/// ```rust,ignore
/// #[derive(UniqueTypeId)]
/// #[UniqueTypeIdFile = "types2.toml"]
/// struct Test1;
/// ```
///
/// ### UniqueTypeIdType
///
/// ```rust,ignore
/// #[derive(UniqueTypeId)]
/// #[UniqueTypeIdType = "i16"]
/// struct Test;
/// ```
///
/// ### UniqueTypeIdStart
///
/// ```rust,ignore
/// #[derive(UniqueTypeId)]
/// #[UniqueTypeIdStart = "23"]
/// struct Test;
/// ```
#[proc_macro_derive(
    UniqueTypeId,
    attributes(UniqueTypeIdFile, UniqueTypeIdType, UniqueTypeIdStart)
)]
pub fn unique_type_id(input: TokenStream) -> TokenStream {
    implement_type_id(input, unique_implementor)
}

fn read_file_into_string(file_name: &str) -> String {
    use std::io::Read;

    let mut f = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => return String::default(),
    };
    f.lock_exclusive().expect("Unable to lock the file");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Unable to read the file");
    f.unlock().expect("Unable to unlock the file");
    contents
}

fn file_string_to_tree(file_contents: String) -> PairsMap {
    let mut map = PairsMap::new();
    file_contents
        .split(&['\n', '\r'][..])
        .filter_map(pair_from_line)
        .for_each(|p| {
            map.insert(p.0, p.1);
        });
    map
}

fn pair_from_line(line: &str) -> Option<(String, u64)> {
    let mut pair = line.split('=');
    let key = pair.next()?.to_owned();
    let value = pair.next()?.parse::<u64>().ok()?;
    Some((key, value))
}

fn append_pair_to_file(file_name: &str, record: &str, value: u64) {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut f = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(file_name)
        .expect("Unable to create file");
    f.lock_exclusive().expect("Unable to lock the file");
    let contents = format!("{}={}\n", record, value);
    f.write_all(contents.as_bytes())
        .expect("Unable to write to the file");
    f.unlock().expect("Unable to unlock the file");
}

fn gen_id(file_name: &str, record: &str, start: u64) -> u64 {
    let pairs_map = file_string_to_tree(read_file_into_string(file_name));
    match pairs_map.get(record) {
        Some(record_id) => record_id.to_owned(),
        None => {
            let mut new_id = start;

            loop {
                if !pairs_map.values().any(|id| &new_id == id) {
                    break;
                }
                new_id += 1;
            }

            append_pair_to_file(file_name, record, new_id);
            new_id
        }
    }
}

fn implement_type_id(
    input: TokenStream,
    implementor: fn(&syn::DeriveInput) -> TokenStream,
) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    implementor(&ast)
}

fn parse_attribute(attrs: &[syn::Attribute], name: &str, default: &str) -> String {
    use quote::ToTokens;
    use syn::spanned::Spanned;

    attrs
        .iter()
        .find(|a| a.path().is_ident(name))
        .map(|a| {
            a.meta
                .to_token_stream()
                .into_iter()
                // Taking the second part of tokens, after the `=` sign.
                .nth(2)
                .ok_or_else(|| {
                    syn::Error::new(
                        a.span(),
                        format!(
                            r#"The attribute should be in the format: `{} = "{}"`"#,
                            name, default
                        ),
                    )
                })
                .unwrap()
                .to_string()
                .trim_matches('\"')
                .to_owned()
        })
        .unwrap_or_else(|| default.to_string())
}

fn unique_implementor(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let types_file_name = parse_attribute(&ast.attrs, "UniqueTypeIdFile", DEFAULT_TYPES_FILE_NAME);
    let id_type_name = parse_attribute(&ast.attrs, "UniqueTypeIdType", DEFAULT_ID_TYPE);
    let gen_start: u64 = parse_attribute(&ast.attrs, "UniqueTypeIdStart", DEFAULT_ID_START)
        .parse()
        .unwrap();
    let id_type = syn::parse_str::<syn::Type>(&id_type_name).unwrap();
    let id = gen_id(&types_file_name, &ast.ident.to_string(), gen_start);

    // TODO: Use TryFrom instead of `#id as #id_type` to avoid silently destructive casts
    TokenStream::from(quote! {
        impl #impl_generics unique_type_id::UniqueTypeId<#id_type> for #name #ty_generics #where_clause {
            const TYPE_ID: unique_type_id::TypeId<#id_type> = unique_type_id::TypeId(#id as #id_type);
            fn id() -> unique_type_id::TypeId<#id_type> {
                Self::TYPE_ID
            }
        }

        impl #name #ty_generics #where_clause {
            const fn unique_type_id() -> unique_type_id::TypeId<#id_type> {
                Self::TYPE_ID
            }
        }
    })
}
