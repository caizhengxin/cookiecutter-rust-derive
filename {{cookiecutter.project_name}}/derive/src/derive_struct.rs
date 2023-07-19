use virtue::generate::Generator;
use virtue::parse::Fields;
use virtue::prelude::*;
use crate::attribute::{ContainerAttributes, FieldAttributes};


pub(crate) struct DeriveStruct {
    pub fields: Option<Fields>,
    pub attributes: ContainerAttributes,
}


impl DeriveStruct {
    pub fn generate_{{cookiecutter.derive_name_slug}}(&self, generator: &mut Generator) -> Result<()> {
        let crate_name = format!("{}::XXXTrait", self.attributes.crate_name);        

        generator
            .impl_for(crate_name)
            .generate_fn("function_name")
            .with_self_arg(FnSelfArg::RefSelf)
            .with_arg("args_name", "args_type")
            .with_return_type("bool")
            .body(|fn_body| {
                fn_body.group(Delimiter::Brace, |fields_body| {
                    if let Some(fields) = self.fields.as_ref() {
                        for field in fields.names() {
                            let attributes = field.attributes().get_attribute::<FieldAttributes>()?.unwrap_or_default();

                        }

                        fields_body.push_parsed("return true")?;
                    }

                    Ok(())
                })?;

                fn_body.push_parsed("false")?;

                Ok(())
            })?;

        Ok(())
    }
}
