#![recursion_limit = "128"]
extern crate fs2;
extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate unique_type_id;

use std::collections::BTreeMap;
use std::fs::File;

use fs2::FileExt;
use proc_macro::TokenStream;

static DEFAULT_TYPES_FILE_NAME: &'static str = "types.toml";
static DEFAULT_ID_TYPE: &'static str = "u64";
static DEFAULT_ID_START: &'static str = "0";

type PairsMap = BTreeMap<String, u64>;

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
        .split('\n')
        .map(pair_from_line)
        .filter(Option::is_some)
        .map(Option::unwrap)
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
        .write(true)
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
                if !pairs_map.values().find(|id| &new_id == *id).is_some() {
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
    implementor(&ast).into()
}

fn parse_attribute(attrs: &[syn::Attribute], name: &str, default: &str) -> String {
    use syn::spanned::Spanned;

    attrs
        .iter()
        .find(|a| a.path.is_ident(name))
        .map(|a| {
            a.tokens
                .clone()
                .into_iter()
                // Taking the second part of tokens, after the `=` sign.
                .nth(1)
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
    })
}
