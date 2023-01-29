use engineer::*;

#[allow(dead_code)]
#[derive(Debug, Engineer)]
#[engineer(builder_func = "new", str_retype)]
struct User {
    id: usize,
    username: String,
    first_name: String,
    #[engineer(default_value = "\"fa\".to_string()")]
    lang_code: Option<String>,
    #[engineer(default)]
    error_code: Option<i8>,
}

#[allow(dead_code)]
#[derive(Debug, Engineer)]
#[engineer(builder_func = "new", str_retype)]
struct Identity {
    id: usize,
    username: String,
    first_name: String,
    last_name: Option<String>,
    #[engineer(default_value = r#"String::from("fa")"#)]
    lang_code: Option<String>,
}

fn print_identity(ident: impl Into<Identity>) {
    let ident: Identity = ident.into();
    println!("{ident:#?}");
}

fn main() {
    let user_1: User = User::new(0_usize, "immmdreza", "MohammadReza").into();

    let user_2 = User::new(1_usize, "jwfly", "Jwfly").lang_code("en").done();

    let ident = Identity::new(1, "immmdreza", "Arash").last_name("Tofani");

    print_identity(ident);

    println!("User 1: {:?}", user_1);
    // User 1: User { id: 0, username: "immmdreza", first_name: "MohammadReza", lang_code: Some("fa"), error_code: Some(0) }

    println!("User 2: {:?}", user_2);
    // User 2: User { id: 1, username: "jwfly", first_name: "Jwfly", lang_code: Some("en"), error_code: Some(0) }
}
