use {{cookiecutter.project_slug}}::{{cookiecutter.derive_name}};
use {{cookiecutter.project_slug}}_derive::{{cookiecutter.derive_name}};


#[derive(Debug, {{cookiecutter.derive_name}})]
pub struct Example {
    pub a: u16,
}


#[test]
fn test_{{cookiecutter.project_slug}}() {
    println!("{:?}", Example { a:1 }.hello_world());
}
