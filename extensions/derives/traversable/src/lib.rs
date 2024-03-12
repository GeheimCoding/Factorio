use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, Meta};

#[proc_macro_derive(Traversable, attributes(lua_object))]
pub fn derive_traversable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let traversals = match &data {
        Data::Struct(data_struct) => generate_traversals_for_struct(data_struct),
        Data::Enum(data_enum) => generate_traversals_for_enum(data_enum),
        _ => quote!(vec![]),
    };
    let is_lua_object = is_lua_object(&data);
    let to_lua_object_impl = generate_to_lua_object_impl(is_lua_object);
    let lua_object_class_id_impl = generate_lua_object_class_id_impl(&ident, is_lua_object);

    (quote! {
        #lua_object_class_id_impl
        impl extensions::Traversable for #ident {
            fn traverse(&self) -> Vec<&dyn extensions::Traversable> {
                #traversals
            }
            #to_lua_object_impl
            fn to_trait_object(&self) -> &dyn extensions::Traversable {
                self
            }
        }
    })
    .into()
}

fn generate_traversals_for_struct(data_struct: &DataStruct) -> proc_macro2::TokenStream {
    let Fields::Named(fields) = &data_struct.fields else {
        return quote!(vec![]);
    };
    generate_traversals_for_fields(fields, true)
}

fn generate_traversals_for_enum(data_enum: &DataEnum) -> proc_macro2::TokenStream {
    if data_enum.variants.is_empty() {
        return quote!(vec![]);
    }
    let variants = data_enum.variants.iter().map(|variant| {
        let ident = &variant.ident;
        if let Fields::Named(fields) = &variant.fields {
            let traversals = generate_traversals_for_fields(fields, false);
            let fields = fields.named.iter().map(|field| &field.ident);
            quote!(Self::#ident {#(#fields),*} => {#traversals})
        } else {
            if variant.fields.is_empty() {
                quote!(Self::#ident => vec![])
            } else if variant.fields.len() == 1 {
                quote!(Self::#ident(value) => vec![value])
            } else {
                unimplemented!()
            }
        }
    });
    quote! {
        match self {
            #(#variants),*
        }
    }
}

fn generate_traversals_for_fields(
    fields_named: &FieldsNamed,
    with_self_prefix: bool,
) -> proc_macro2::TokenStream {
    let prefix = if with_self_prefix {
        quote!(self.)
    } else {
        quote!()
    };
    let fields = fields_named.named.iter().map(|field| &field.ident);
    quote! {
        vec![#(#prefix #fields.to_trait_object(),)*]
    }
}

fn is_lua_object(data: &Data) -> bool {
    let Data::Struct(data_struct) = data else {
        return false;
    };
    data_struct.fields.iter().any(|field| {
        matches!(&field.ident, Some(ident) if ident == "class_id") && field.attrs.iter().any(|attribute| {
            matches!(&attribute.meta, Meta::Path(path) if path.segments.iter().any(|segment| segment.ident == "lua_object"))
        })
    })
}

fn generate_lua_object_class_id_impl(
    ident: &Ident,
    is_lua_object: bool,
) -> proc_macro2::TokenStream {
    if is_lua_object {
        quote! {
            impl extensions::LuaObjectClassId for #ident {
                fn class_id(&self) -> &str {
                    &self.class_id
                }
            }
        }
    } else {
        quote!()
    }
}

fn generate_to_lua_object_impl(is_lua_object: bool) -> proc_macro2::TokenStream {
    let option = if is_lua_object {
        quote!(Some(self))
    } else {
        quote!(None)
    };
    quote! {
        fn to_lua_object(&self) -> Option<&dyn extensions::LuaObject> {
            #option
        }
    }
}
