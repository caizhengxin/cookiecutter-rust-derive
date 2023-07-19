// #![feature(let_chains)]
extern crate proc_macro;

mod {{cookiecutter.derive_attribute_name}};

use {{cookiecutter.derive_attribute_name}}::attribute::ContainerAttributes;
use {{cookiecutter.derive_attribute_name}}::derive_enum;
use {{cookiecutter.derive_attribute_name}}::derive_struct;

use proc_macro::TokenStream;
use virtue::prelude::*;


#[proc_macro_derive({{cookiecutter.derive_name}}, attributes({{cookiecutter.derive_attribute_name}}))]
pub fn derive_{{cookiecutter.derive_name_slug}}(input: TokenStream) -> TokenStream {
    derive_{{cookiecutter.derive_name_slug}}_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_{{cookiecutter.derive_name_slug}}_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, attributes, body) = parse.into_generator();
    let attributes = attributes
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(body) => {
            derive_struct::DeriveStruct {
                fields: body.fields,
                attributes,
            }.generate_{{cookiecutter.derive_name_slug}}(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_{{cookiecutter.derive_name_slug}}(&mut generator)?;
        }
    }

    generator.export_to_file("{{cookiecutter.derive_attribute_name}}", "{{cookiecutter.derive_name}}");
    generator.finish()
}
