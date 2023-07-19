#![feature(let_chains)]

#[cfg(feature = "{{cookiecutter.project_slug}}_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate {{cookiecutter.project_slug}}_derive;

#[cfg(feature = "{{cookiecutter.project_slug}}_derive")]
pub use {{cookiecutter.project_slug}}_derive::{{cookiecutter.derive_name}};
