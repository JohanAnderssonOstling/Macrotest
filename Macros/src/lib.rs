extern crate proc_macro2;

use proc_macro2::{Span, TokenStream};
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemStruct, Fields, ItemImpl, ImplItem, parse::Parse, parse::ParseStream, Result};

struct MyStruct {
    item_struct: ItemStruct, 	item_impl: ItemImpl,
}

impl Parse for MyStruct {
    fn parse(input: ParseStream) -> Result<Self> {
	let item_struct: ItemStruct = input.parse()?; 		// Parse input as a struct definition
	let item_impl: ItemImpl = input.parse()?; 		// Parse input as an impl block
	Ok(Self { item_struct, item_impl })
    }
}

fn generate_methods(methods: Vec<ImplItem>) -> Vec<TokenStream> {
    methods.into_iter().filter_map(|item| {
	if let ImplItem::Fn(method) = item {
	    let method_args = &method.sig.inputs; // Inputs for the function
	    let return_type = &method.sig.output;
	    let name = &method.sig.ident; 		// Original function name
	    let new_name = format_ident!("_{}", name); 	// Add underscore prefix to function name
	    let block = &method.block; 		// Function body

	    // Add underscore prefix to existing methods
	    let new_method_with_underscore = quote! {
                fn #new_name(#method_args) #return_type
		    #block
            };

	    // Create wrapper functions
	    let new_method_with_empty_body = quote! {
                fn #name(#method_args) #return_type {}
            };

	    Some(vec![new_method_with_underscore, new_method_with_empty_body])
	} else {
	    None // Ignore non-method items in the impl block
	}
    }).flatten().collect()
}

#[proc_macro]
pub fn offload(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let MyStruct { item_struct, item_impl } = parse_macro_input!(input as MyStruct);

    let name = &item_struct.ident;
    let methods = generate_methods(item_impl.items);

    let gen = match &item_struct.fields {
	Fields::Named(named) => {
	    let fields: Vec<_> = named.named.iter().collect();

	    quote! {
                pub struct #name {
                    #(#fields,)*
                    offload: bool
                }

                impl #name {
                    #(#methods)*
                }
            }
	},
	_ => {
	    TokenStream::from(quote! {compile_error!("Only named fields are supported");})
	}
    };

    gen.into()
}




/*



#[proc_macro_derive(Offloadable)]
pub fn make_offloadable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);	// Parse the input tokens into a syntax tree
    let mut fields = vec![];

    let Data::Struct(DataStruct { fields: Fields::Named(fields_named), .. }): Data = input.data;

    for field in &fields_named.named {			//Add existing fields
	fields.push(quote! { #field, });
    }

    let ident = input.ident;
    let output = quote! {
        pub struct #input.ident {
            #(#fields)*
            is_offloaded: bool,
        }
    };

    output.into()
}
*/
/*
#[proc_macro_attribute]
pub fn wrap_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemImpl);		// Parse the input tokens into a syntax tree
    let struct_name = &ast.self_ty;				// Get name of struct that is implemented
    let offloadable_struct = generate_struct(struct_name);

    let mut methods = vec![];

    for item in ast.items {
        if let syn::ImplItem::Method(method) = item {
	  let offloadable_methods = generate_method(&method);
	  methods.push(offloadable_methods);
        }
    }

    let offloadable_struct_ident = struct_name.clone();

    let offload = quote! {
        pub fn offload(&mut self) {
            self.is_offloaded = true;
        }
    };

    let deoffload = quote! {
        pub fn deoffload(&mut self) {
            self.is_offloaded = false;
        }
    };

    let output = quote! {
        #offloadable_struct

        impl #offloadable_struct_ident {
            #(#methods)*

            // Add offload() and deoffload() methods
            #offload
            #deoffload        }
    };

    output.into()
}

fn generate_struct(struct_name: &Type) -> impl ToTokens {
    let offloadable_struct_ident = struct_name.clone();

    quote! {
        pub struct #offloadable_struct_ident {
            inner: #struct_name,
            is_offloaded: bool,
        }
    }
}

fn generate_method(method: &syn::ImplItemMethod) -> impl ToTokens {
    let fn_name = &method.sig.ident;
    let fn_inputs = method.sig.inputs.iter().map(|arg| {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
	  quote! { #pat: #ty }
        } else {
	  quote! {}
        }
    });

    let fn_output = &method.sig.output;

    quote! {
        fn #fn_name(&self, #(#fn_inputs),*) -> #fn_output {
            self.inner.#fn_name(#(#fn_inputs),*)
        }
    }
}
*/