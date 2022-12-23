use proc_macro::{TokenStream};
use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields};


macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

#[proc_macro_derive(NetworkSerializable)]
pub fn derive_is_variant(input: TokenStream) -> TokenStream {
    // See https://doc.servo.org/syn/derive/struct.DeriveInput.html
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    
    // get enum name
    let ref name = input.ident;
    let ref data = input.data;

    // data is of type syn::Data
    // See https://doc.servo.org/syn/enum.Data.html
    let (size_func, serialize_func, deserialize_func) = match data {
        // Only if data is an enum, we do parsing
        Data::Enum(data_enum) => {

            // data_enum is of type syn::DataEnum
            // https://doc.servo.org/syn/struct.DataEnum.html

            // assign an id to each variant
            let mut variant_id: usize = 0;

            // size function : returns size of the data of the enum
            let mut size_func_internal = TokenStream2::new();
            // serialize function : returns Vec<u8> being the raw data.
            let mut serialize_func_internal = TokenStream2::new();
            // deserialize function : returns Self from id and Vec<u8>
            let mut deserialize_func_internal = TokenStream2::new();

            // Iterate over enum variants
            // `variants` if of type `Punctuated` which implements IntoIterator
            //
            // https://doc.servo.org/syn/punctuated/struct.Punctuated.html
            // https://doc.servo.org/syn/struct.Variant.html
            for variant in &data_enum.variants {

                // Variant's name
                let ref variant_name = variant.ident;

                // Variant can have unnamed fields like `Variant(i32, i64)`
                // Variant can have named fields like `Variant {x: i32, y: i32}`
                // Variant can be named Unit like `Variant`
                let fields_in_variant = match &variant.fields {
                    Fields::Unnamed(_) => quote_spanned! {variant.span()=> (..) },
                    Fields::Unit => quote_spanned! { variant.span()=> },
                    Fields::Named(_) => quote_spanned! {variant.span()=> {..} },
                };

                // size match arm for the current variant (compute size from fields)
                let variant_size = match &variant.fields {
                    Fields::Unit => quote!{std::mem::size_of::<usize>()}, // no fields, message of size usize (id)
                    Fields::Unnamed(fields) => {
                        let mut field_sum = quote!{std::mem::size_of::<usize>()};
                        for field in fields.unnamed.iter() {
                            field_sum.extend(quote!{+ std::mem::size_of::<#field>()})
                        }
                        field_sum
                    },
                    Fields::Named(fields) => {
                        let mut field_sum = quote!{std::mem::size_of::<usize>()};
                        for field in fields.named.iter() {
                            field_sum.extend(quote!{+ std::mem::size_of::<#field>()})
                        }
                        field_sum
                    },
                };
                size_func_internal.extend(quote_spanned! {variant.span()=>
                    #name::#variant_name #fields_in_variant => #variant_size,
                });

                serialize_func_internal.extend(match &variant.fields {
                    Fields::Unit => quote_spanned! {variant.span()=>
                        #name::#variant_name => NetworkSerializable::serialize(#variant_id as usize),
                    }, // no fields, message of size 0
                    Fields::Unnamed(fields) => {
                        let field_name: Vec<_> = fields.unnamed.iter().enumerate().map(|(i, _)| {
                            format_ident!("field{}", i)
                        }).collect();
                        
                        quote_spanned! {variant.span()=>
                            #name::#variant_name (#(#field_name,)*) => {
                                let mut result = Vec::with_capacity(self.size());
                                result.append(&mut NetworkSerializable::serialize(#variant_id as usize));
                        
                                #(
                                    result.append(&mut NetworkSerializable::serialize(#field_name));
                                )*
                        
                                result
                            },
                        }
                    },
                    Fields::Named(fields) => {
                        let field_name: Vec<_> = fields.named.iter().enumerate().map(|(i, _)| {
                            format_ident!("field{}", i)
                        }).collect();
                        
                        quote_spanned! {variant.span()=>
                            #name::#variant_name (#(#field_name,)*) => {
                                let mut result = Vec::with_capacity(self.size());
                                result.append(&mut NetworkSerializable::serialize(#variant_id as usize));
                        
                                #(
                                    result.append(&mut NetworkSerializable::serialize(#field_name));
                                )*
                        
                                result
                            },
                        }
                    },
                });

                deserialize_func_internal.extend(match &variant.fields {
                    Fields::Unit => quote_spanned! {variant.span()=>
                        #variant_id => Ok(#name::#variant_name),
                    }, // no fields, message of size 0
                    Fields::Unnamed(fields) => {
                        let field_name: Vec<_> = fields.unnamed.iter().enumerate().map(|(i, _)| {
                            format_ident!("field{}", i)
                        }).collect();
                        let field_types: Vec<_> = fields.unnamed.iter().collect();
                        
                        quote_spanned! {variant.span()=>
                            #variant_id => {
                                let mut offset = std::mem::size_of::<usize>();;
                        
                                #(
                                    let #field_name = match NetworkSerializable::deserialize((&data[offset..offset+std::mem::size_of::<#field_types>()]).to_vec()) {
                                        Ok(result) => result,
                                        Err(e) => return Err(e),
                                    };
                                    offset += std::mem::size_of::<#field_types>();
                                )*
                        
                                Ok(#name::#variant_name (#(#field_name,)*))
                            },
                        }
                    },
                    Fields::Named(fields) => {
                        let field_name: Vec<_> = fields.named.iter().enumerate().map(|(i, _)| {
                            format_ident!("field{}", i)
                        }).collect();
                        let field_types: Vec<_> = fields.named.iter().collect();
                        
                        quote_spanned! {variant.span()=>
                            #variant_id => {
                                let mut offset = std::mem::size_of::<usize>(); // we read the id !
                                
                                #(
                                    let #field_name = match NetworkSerializable::deserialize((&data[offset..offset+std::mem::size_of::<#field_types>()]).to_vec()) {
                                        Ok(result) => result,
                                        Err(e) => return Err(e),
                                    };
                                    offset += std::mem::size_of::<#field_types>();
                                    derive_error!("here");
                                )*
                                
                                Ok(#name::#variant_name (#(#field_name,)*))
                            },
                        }
                    },
                });

                variant_id += 1;
            }
            (quote!{
                match &self {
                    #size_func_internal
                }
            },
            quote!{
                match self {
                    #serialize_func_internal
                }
            },
            quote!{
                // read enum id in the data !
                let id: usize = match NetworkSerializable::deserialize((&data[0..std::mem::size_of::<usize>()]).try_into().unwrap()) {
                    Ok(result) => result,
                    Err(e) => return Err(NetworkUnserializeError::IncompleteData),
                };
                match id {
                    #deserialize_func_internal
                    _ => Err(NetworkUnserializeError::InvalidId),
                }
            })
            
        },
        Data::Struct(_data_struct) => {
            // todo
            return derive_error!("IsVariant is only implemented for enums (structs incoming tho)");
        },
        _ => return derive_error!("IsVariant is only implemented for enums"),
    };
    
    
    let expanded = quote! {
        impl NetworkSerializable for #name {
            fn size(&self) -> usize {
                #size_func
            }
            fn serialize(self) -> Vec<u8> {
                #serialize_func
            }
            fn deserialize(data: Vec<u8>) -> Result<Self, NetworkUnserializeError> {
                #deserialize_func
            }
            
        }
    };
    
    TokenStream::from(expanded)

}

