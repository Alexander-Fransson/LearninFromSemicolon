use proc_macro::TokenStream;
use syn::DeriveInput;

fn implement_fields_trait(ast: DeriveInput) -> TokenStream {
    let struct_name = ast.ident;
    let ident_str = struct_name.to_string();

    let fields: Vec<syn::Ident> = match ast.data {
        syn::Data::Struct(s) => s.fields.into_iter().filter_map(|f| f.ident).collect(),
        syn::Data::Enum(_) => panic!("Enums are not supported by this macro"),
        syn::Data::Union(_) => panic!("Unions are not supported by this macro")
    };
    let filed_idents_string: Vec<String> = fields.iter().map(|f| f.to_string()).collect::<Vec<String>>();

    quote::quote! {
        impl Fields for #struct_name {
            fn struct_name(&self) -> &'static str {
                #ident_str
            }

            fn fields(&self) -> Vec<&'static str> {
                vec![#(#filed_idents_string),*]
            }
        }
    }.into()
}

#[proc_macro_derive(Fields)]
pub fn fields_derive(input: TokenStream) -> TokenStream {
    // parse the token stream
    let ast: DeriveInput = syn::parse(input).unwrap();

    // generate changes and return modified token stream
    implement_fields_trait(ast)
}