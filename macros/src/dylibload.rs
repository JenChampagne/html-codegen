use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::{spanned::Spanned, FnArg, Ident, ItemFn, LitStr, Pat, PatIdent, PatType};

pub fn wrap_dylibload_function(
    scope: Ident,
    scope_target_dir: LitStr,
    input: ItemFn,
) -> TokenStream {
    let fn_vis = input.vis;
    let fn_name = input.sig.ident;
    let fn_generics = input.sig.generics; // todo

    let fn_inputs = input.sig.inputs;
    let fn_inputs_list = fn_inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(rec) => {
                emit_error!(rec.span(), "Don't use `self` on components");
                panic!();
            }
            FnArg::Typed(PatType { pat, .. }) => match pat.as_ref() {
                Pat::Ident(PatIdent { ident, .. }) => quote! { #ident, },
                _ => unimplemented!(),
            },
        })
        .collect::<Vec<_>>();

    let fn_output = input.sig.output;
    let fn_output_type = match &fn_output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_arrow, ty) => quote! { #ty },
    };

    quote! {
        #fn_vis fn #fn_name #fn_generics ( #fn_inputs )
            -> Result<#fn_output_type, ::html_codegen::dylibload::Error>
        {
            use ::html_codegen::dylibload::{
                Error, Library, Symbol, library_filename
            };

            // Attempt to validate the function signature in code matches
            // the one in the external crate.
            let _: fn( #fn_inputs ) #fn_output = ::#scope::#fn_name;

            // `debug_assertions` allows us to detect debug vs release builds.
            #[cfg(debug_assertions)]
            {
                println!(">>> in debug looking at {:?}.", library_filename( stringify!( #scope ) ));
//*
    let base_dir = concat!( #scope_target_dir, "" );
    let output = String::from_utf8(
        std::process::Command::new("find")
            .args([base_dir, "-name", concat!("lib", stringify!( #scope ), "-d*.so")])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let mut items = output.lines().collect::<Vec<&str>>();
    println!(">>>> LIBx: {base_dir} - {items:#?}");
    items.sort();
    let output = items.iter().last().unwrap();
    println!(">>>> LIB: {output:?}");
    //println!(">>>> ENV: {:#?}", std::env::vars());
//*/
                // Try to load the `html` crate.
                let lib = unsafe {
                    // Library::new( library_filename( stringify!( #scope ) ) )?
                    // Library::new( concat!( #scope_target_dir, "/debug/lib", stringify!( #scope ), ".so" ) )?
                    Library::new( output )?
                };
                let func = unsafe {
                    lib.get::<Symbol< fn( #fn_inputs ) #fn_output >>(
                        stringify!( #fn_name ).as_bytes()
                    )?
                };

                let result = func( #(#fn_inputs_list)* );

                lib.close().unwrap();

                Ok(result)
            }

            // This is the simplified version for release builds.
            #[cfg(not(debug_assertions))]
            {
                println!(">>> in release looking at {:?}.", library_filename( stringify!( #scope ) ));
                Ok(::#scope::#fn_name( #(#fn_inputs_list)* ))
            }
        }
    }
    .into()
}
