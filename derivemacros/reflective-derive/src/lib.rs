
// created by doing cargo new from within the parrent project
// manually changed name to lib 
// maybe cargo new --lib would make it a lib by default

use proc_macro::TokenStream;
use syn::DeriveInput;

fn implement_reflective_trait(ast: DeriveInput) -> TokenStream {
    // get struct identifier
    let struct_name = ast.ident;
    let ident_str = struct_name.to_string();

    // get struct fields
    #[allow(unused)]
    let fields: Vec<syn::Ident> = match ast.data {
        syn::Data::Struct(s) => s.fields.into_iter().filter_map(|f| f.ident).collect(),
        syn::Data::Enum(_) => panic!("Enums are not supported by this macro"),
        syn::Data::Union(_) => panic!("Unions are not supported by this macro"),
        _ => panic!("Derive macro can only be used on structs"),
    };
    let filed_idents_string: Vec<String> = fields.iter().map(|f| f.to_string()).collect::<Vec<String>>();

    // generate impl
    quote::quote! {
        impl Reflective for #struct_name {
            fn name(&self) -> &'static str {
                #ident_str // use # before variables from outside quote to access them
            }

            fn fields(&self) -> Vec<&'static str> {
                vec![#(#filed_idents_string),*] // unpacks the vector into a comma separated string
            }
        }
    }
    .into()
}

#[proc_macro_derive(GetName)]// creates the name, can be set to anything
pub fn reflective_derive(input: TokenStream) -> TokenStream {
    // parse the token stream
    let ast: DeriveInput = syn::parse(input).unwrap();

    // generate changes and return modified token stream
    implement_reflective_trait(ast)
}