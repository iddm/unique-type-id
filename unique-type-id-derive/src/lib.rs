#![recursion_limit="128"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate unique_type_id;
extern crate fs2;

use std::fs::File;

use proc_macro::TokenStream;
use fs2::FileExt;

static TYPES_FILE_NAME: &'static str = "types.toml";

#[proc_macro_derive(SequentialTypeId, attributes(UniqueTypeIdFile))]
pub fn sequential_type_id(input: TokenStream) -> TokenStream {
    implement_type_id(input, sequential_implementor)
}

fn read_file_into_string(file_name: &str) -> String {
    use std::io::Read;

    let mut f = File::open(file_name).expect("File not found");
    f.lock_exclusive().expect("Unable to lock the file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Unable to read the file");
    f.unlock().expect("Unable to unlock the file");
    contents
}

fn append_pair_to_file(file_name: &str, record: &str, value: u64) {
    use std::io::Write;
    use std::fs::OpenOptions;

    let mut f = OpenOptions::new().write(true).append(true).open(file_name).expect("Unable to create file");
    f.lock_exclusive().expect("Unable to lock the file");
    let contents = format!("{}={}\n", record, value);
    f.write_all(contents.as_bytes()).expect("Unable to write to the file");
    f.unlock().expect("Unable to unlock the file");
}

fn record_in_pair(record: &str, pair_string: &str) -> bool {
    if let Some(r) = pair_string.split('=').next() {
        return r == record
    }

    false
}

fn value_from_pair(pair_string: &str) -> Option<u64> {
    pair_string.split('=').nth(1).unwrap_or("").parse::<u64>().ok()
}

fn find_record(file_contents: &str, record: &str) -> Option<u64> {
    let mut lines = file_contents.split('\n');
    lines.find(|s| record_in_pair(record, s)).map(value_from_pair)?
}

fn last_id(file_contents: &str) -> u64 {
    let mut last_id = 0;
    let lines = file_contents.split('\n');
    lines.for_each(|line| last_id = std::cmp::max(last_id, value_from_pair(line).unwrap_or(0)));
    last_id
}

fn gen_id(file_name: &str, record: &str) -> u64 {
    let file_contents = read_file_into_string(file_name);
    let id = match find_record(&file_contents, record) {
        Some(id) => id,
        None => {
            let new_id = last_id(&file_contents) + 1;
            append_pair_to_file(file_name, record, new_id);
            new_id
        },
    };
    id
}

fn implement_type_id(input: TokenStream, implementor: fn(&syn::DeriveInput) -> quote::Tokens) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = implementor(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn parse_types_file_name(attrs: &[syn::Attribute]) -> String {
    let name = attrs.iter().filter(|a| a.value.name() == "UniqueTypeIdFile").next();
    if let Some(name) = name {
        if let syn::MetaItem::NameValue(_, ref value) = name.value {
            match value {
                &syn::Lit::Str(ref value, _) => return value.to_owned(),
                _ => {},
            }
        }
    }
    TYPES_FILE_NAME.to_owned()
}


fn sequential_implementor(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let types_file_name = parse_types_file_name(&ast.attrs);
    let id = gen_id(&types_file_name, name.as_ref());

    quote! {
        impl #impl_generics unique_type_id::SequentialTypeId for #name #ty_generics #where_clause {
            fn id() -> unique_type_id::TypeId {
                unique_type_id::TypeId(#id)
            }
        }
    }
}
