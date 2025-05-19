extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Item, ItemFn, ItemImpl,
    ItemStruct, ImplItem
};
use libc::c_char;

fn handle_function(item_fn: &ItemFn) -> TokenStream {
    let fn_name = &item_fn.sig.ident;
    let fn_name_str = fn_name.to_string();

    let params_str = item_fn
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => {
                let name = match &*pat_type.pat {
                    syn::Pat::Ident(ident) => ident.ident.to_string(),
                    _ => "_".into(),
                };
                let ty = quote!(#pat_type.ty).to_string();
                format!("{}", ty)
            }
            _ => "".to_string(),
        })
        .collect::<Vec<_>>()
        .join(", ");

    let return_type_str = match &item_fn.sig.output {
        syn::ReturnType::Default => "".to_string(),
        syn::ReturnType::Type(_, ty) => format!(" -> {}", quote!(#ty)),
    };

    let signature = format!("fn {}({}){}", fn_name_str, params_str, return_type_str);
    let register_fn = format_ident!("__register_{}", fn_name);

    let wrapped = quote! {
        #[no_mangle]
        pub extern "C" #item_fn

        #[ctor::ctor]
        fn #register_fn() {
            rew_bindgen::registry::register_function(#fn_name_str, #signature);
        }
    };

    wrapped.into()
}

fn handle_struct(item_struct: &ItemStruct) -> TokenStream {
    let struct_name = &item_struct.ident;
    let struct_name_str = struct_name.to_string();

    let field_names = item_struct
        .fields
        .iter()
        .filter_map(|f| f.ident.as_ref().map(|id| id.to_string()))
        .collect::<Vec<_>>();

    let register_fn = format_ident!("__register_struct_{}", struct_name);

    let wrapped = quote! {
        #item_struct

        #[ctor::ctor]
        fn #register_fn() {
            rew_bindgen::registry::register_struct(#struct_name_str, vec![#(#field_names.to_string()),*]);
        }
    };

    wrapped.into()
}

fn handle_impl_block(item_impl: &ItemImpl) -> TokenStream {
    let self_ty = &item_impl.self_ty;
    let struct_name = quote!(#self_ty).to_string().replace(' ', "");

    let mut registrations = Vec::new();
    let mut methods = Vec::new();

    for item in &item_impl.items {
        if let syn::ImplItem::Fn(method) = item {
            let method_name = method.clone().sig.ident.to_string();
            let full_name = format!("{}::{}", struct_name, method_name);

            let params_str = method
                .sig
                .inputs
                .iter()
                .map(|arg| match arg {
                    syn::FnArg::Typed(pat_type) => {
                        let name = match &*pat_type.pat {
                            syn::Pat::Ident(ident) => ident.ident.to_string(),
                            _ => "_".into(),
                        };
                        let ty = quote!(#pat_type.ty).to_string();
                        format!("{}: {}", name, ty)
                    }
                    syn::FnArg::Receiver(r) => {
                        if r.reference.is_some() {
                            if r.mutability.is_some() {
                                "&mut self".to_string()
                            } else {
                                "&self".to_string()
                            }
                        } else {
                            "self".to_string()
                        }
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");

            let return_type_str = match &method.sig.output {
                syn::ReturnType::Default => "".to_string(),
                syn::ReturnType::Type(_, ty) => format!(" -> {}", quote!(#ty)),
            };

            let signature = format!("fn {}({}){}", method_name, params_str, return_type_str);

            let register_fn = format_ident!("__register_method_{}_{}", struct_name, method_name);
            registrations.push(quote! {
                #[ctor::ctor]
                fn #register_fn() {
                    rew_bindgen::registry::register_function(#full_name, #signature);
                }
            });

            methods.push(method.clone());
        }
    }

    let output = quote! {
        #item_impl
        #(#registrations)*
    };

    output.into()
}


#[proc_macro_attribute]
pub fn rew_export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);

    match &item {
        Item::Fn(item_fn) => handle_function(item_fn),
        Item::Struct(item_struct) => handle_struct(item_struct),
        Item::Impl(item_impl) => handle_impl_block(item_impl),
        _ => {
            syn::Error::new_spanned(item, "Only functions and structs can be annotated with #[rew_export]")
                .to_compile_error()
                .into()
        }
    }
}
