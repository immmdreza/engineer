use engineer::*;

#[allow(dead_code)]
#[derive(Debug, Engineer)]
#[engineer(engineer_name = "UserBuilder", builder_func = "new")]
struct User {
    id: usize,
    #[engineer(str_retype)]
    username: String,
    #[engineer(str_retype)]
    first_name: String,
    #[engineer(str_retype, default = "\"fa\".to_string()")]
    lang_code: Option<String>,
    fang_code: Option<String>,
}

fn main() {
    let user_1 = User::new(0, "immmdreza", "MohammadReza").done();

    let user_2 = User::new(1, "jwfly", "Jwfly").lang_code("en").done();

    println!("User 1: {:?}", user_1);
    // User 1: User { id: 0, username: "immmdreza", first_name: "MohammadReza", lang_code: Some("fa"), fang_code: None }

    println!("User 2: {:?}", user_2);
    // User 2: User { id: 1, username: "jwfly", first_name: "Jwfly", lang_code: Some("en"), fang_code: None }
}
