mod backend {
    use engineer::*;

    #[allow(dead_code)]
    #[derive(Debug, Engineer)]
    #[engineer(new, str_retype)]
    pub struct User {
        id: usize,
        username: String,
        first_name: String,
        #[engineer(default_value = r#"String::from("fa")"#)]
        lang_code: Option<String>,
        #[engineer(default)]
        error_code: Option<i8>,
    }

    #[allow(dead_code)]
    #[derive(Debug, Engineer)]
    #[engineer(new, str_retype)]
    pub struct Identity {
        id: usize,
        username: String,
        first_name: String,
        #[engineer(default_value = r#"String::from("Tofani")"#)]
        last_name: String,
        #[engineer(default_value = r#"String::from("fa")"#)]
        lang_code: Option<String>,
    }
}

use backend::*;
use engineer::*;

fn print_identity(ident: impl Into<Identity>) {
    let ident: Identity = ident.into();
    println!("{ident:#?}");
}

fn build_any<E>(required: E::Params) -> E
where
    E: Engineer,
{
    E::build(required)
}

#[allow(dead_code)]
fn get_builder<E>(required: E::Params) -> E::Builder
where
    E: Engineer,
{
    E::builder(required)
}

fn main() {
    let user_1: User = User::new(0_usize, "immmdreza", "MohammadReza").into();

    // Using default Engineer impl (params are passed as a tuple)
    let user_2 = User::builder((1_usize, "jwfly".to_string(), "Jwfly".to_string()))
        .lang_code("d")
        .done();

    let _ = User::build_default();

    let ident = Identity::new(1, "immmdreza", "Arash");

    print_identity(ident);

    let _: User = build_any((0_usize, "immmdreza".to_string(), "MohammadReza".to_string()));

    println!("User 1: {:?}", user_1);
    // User 1: User { id: 0, username: "immmdreza", first_name: "MohammadReza", lang_code: Some("fa"), error_code: Some(0) }

    println!("User 2: {:?}", user_2);
    // User 2: User { id: 1, username: "jwfly", first_name: "Jwfly", lang_code: Some("en"), error_code: Some(0) }
}
