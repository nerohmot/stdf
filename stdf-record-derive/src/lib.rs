extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(STDFRecord, attributes(array_length, nibble_array_length))]
pub fn stdf_record(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let name = derive_input.ident;
    let generics = derive_input.generics;
    let record_struct = if let syn::Data::Struct(ref record_struct) = derive_input.data {
        record_struct
    } else {
        return TokenStream::from(quote!{});
    };

    let try_read_vars = record_struct.fields.iter().map(|ref x| {
        let name = x.ident.as_ref().unwrap();
        let ty = &x.ty;
        quote! {
            let #name = bytes.read_with::<#ty>(offset, endian)?;
        }
    });
    let try_read_fields = record_struct.fields.iter().map(|ref x| {
        let name = x.ident.as_ref().unwrap();
        quote! {
            #name: #name
        }
    });
    let try_write_fields = record_struct.fields.iter().map(|ref x| {
        let name = x.ident.as_ref().unwrap();
        let ty = &x.ty;
        quote! {
            bytes.write_with::<#ty>(offset, self.#name, endian)?;
        }
    });
    let default_impl_generics: syn::Generics = parse_quote! { <'a> };
    let (record_impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    // TryRead needs to declare an 'a lifetime for the byte slice. The record type we're
    // implementing for might already declare an 'a lifetime if it contains variable-length field
    // types, which should use the same lifetime as the read buffer. However if it doesn't, an 'a
    // lifetime needs to be declared.
    let (impl_generics, record_ty_lifetimes) = if generics.lifetimes().count() == 0 {
        let (ig, _, _) = default_impl_generics.split_for_impl();
        (ig, default_impl_generics.lifetimes())
    } else {
        (record_impl_generics, generics.lifetimes())
    };
    let try_read = quote!{
        impl #impl_generics TryRead<#(#record_ty_lifetimes,)* ctx::Endian> for #name #ty_generics #where_clause {
            fn try_read(bytes: &'a [u8], endian: ctx::Endian) -> byte::Result<(Self, usize)> {
                let offset = &mut 0;
                #(#try_read_vars)*
                Ok((
                    #name {
                        #(#try_read_fields),*
                    },
                    *offset,
                ))
            }
        }
    };
    let try_write = quote!{
        impl #impl_generics TryWrite<ctx::Endian> for #name #ty_generics #where_clause {
            fn try_write(self, bytes: &mut [u8], endian: ctx::Endian) -> byte::Result<usize> {
                let offset = &mut 0;
                #(#try_write_fields);*
                Ok(*offset)
            }
        }
    };
    TokenStream::from(quote!{
        #try_read
        #try_write
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
