# Engineer

Engineer is a master builder based on `Optional`.

It just generates an Engineer (Builder) class for a data model.

## Installation

Add following as dependencies

```toml
[dependencies]
engineer = "0.1.3"
```

## Get Started

```rust
use engineer::Engineer;

#[derive(Engineer)]
struct Identity {
    id: usize,
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
    lang_code: Option<String>,
}
```

Optional fields are not required during the initialization.

```rust
// Option fields are set to None.
let identity = Identity::engineer(0, "immmdreza".to_string()).done();
```

But you can set a value for `Option` fields as well.

```rust
let identity = Identity::engineer(0, "immmdreza".to_string()) // IdentityEngineer
    .first_name("Arash".to_string()) // IdentityEngineer
    .last_name("Tofani".to_string()) // IdentityEngineer
    .done(); // Identity
```

That's all for the basics, but you can do a little customizations.

## Customizations

### Engineer Struct Name

Engineer struct name is {struct}Engineer (`IdentityEngineer` for `Identity`) by default, but you can change that.

```rust
// ~~~ sniff ~~~

#[derive(Engineer)]
#[engineer(engineer_name = "IdentityBuilder")]
struct Identity {
    // ~~~ sniff ~~~
}

// ~~~ sniff ~~~

    let identity = Identity::engineer(0, "immmdreza".to_string()) // IdentityBuilder
        .first_name("Arash".to_string()) // IdentityBuilder
        .last_name("Tofani".to_string()) // IdentityBuilder
        .done(); // Identity
```

### Builder Function Name

The name of builder function is `engineer` by default, but guess what?

```rust
// ~~~ sniff ~~~

#[derive(Engineer)]
#[engineer(builder_func = "builder")]
struct Identity {
    // ~~~ sniff ~~~
}

// ~~~ sniff ~~~

    let identity = Identity::builder(0, "immmdreza".to_string())
    // ~~~ sniff ~~~
```

You want to use this as `new` function:

```rust
// ~~~ sniff ~~~

#[derive(Engineer)]
#[engineer(builder_func = "new")]
struct Identity {
    // ~~~ sniff ~~~
}

// ~~~ sniff ~~~

    let identity = Identity::new(0, "immmdreza".to_string())
    // ~~~ sniff ~~~
```

This can be simplified for special `new` name as builder function:

```rust
#[derive(Engineer)]
#[engineer(new)]
struct Identity {
    // ~~~ sniff ~~~
}
```

### Default value for Options

You can set a default value for option fields.

This value is used if you don't set any other for them.

```rust
// ~~~ sniff ~~~

#[derive(Engineer)]
#[engineer(new)]
struct Identity {
    // ~~~ sniff ~~~
    #[engineer(default_value = r#"String::from("fa")"#)]
    lang_code: Option<String>,
}

// ~~~ sniff ~~~

    let identity = Identity::new(0, "immmdreza".to_string());

    identity.lang_code // Some("fa")
```

Alternatively, you can use `default` to set `Some(Default::default)` instead of None if any other value is not given.

```rust
    // ~~~ sniff ~~~
    #[engineer(default)]
    luck_number: Option<i32>, // Some(0)
    // ~~~ sniff ~~~
```

### Retype

You can change types requested in builder processes.

```rust
// ~~~ sniff ~~~

#[derive(Engineer)]
#[engineer(new)]
struct Identity {
    // ~~~ sniff ~~~
    #[engineer(retype(to = "impl Into<String>", re = ".into()"))]
    //                      ^                         ^
    //                      | Requested type in public.
    //                                                |
    //                                                | How we recover to original type.
    username: String,
    // ~~~ sniff ~~~
}

// ~~~ sniff ~~~

    let identity = Identity::new(0, "immmdreza"); // .to_string() is not needed.
    // ~~~ sniff ~~~
```

Alternatively, for str retypes (like example above), you can use a shorthand `str_retype`.

```rust
    // ~~~ sniff ~~~
    #[engineer(str_retype)]
    username: String,
    // ~~~ sniff ~~~
```

Also you can use retypes globally.

```rust
#[derive(Engineer)]
#[engineer(new, retype(from = "String", to = "impl Into<String>", re = ".into()"))]
struct Identity {
    // ~~~ sniff ~~~
}
```

Or additionally for String retypes:

```rust
#[derive(Engineer)]
#[engineer(new, str_retype)]
struct Identity {
    // ~~~ sniff ~~~
}
```

Both codes above will retype **all** `String` fields into `impl Into<String>` in public api.

Final result

```rust
#[derive(Engineer)]
#[engineer(new, str_retype)]
struct Identity {
    id: usize,
    username: String,
    first_name: String,
    last_name: Option<String>,
    #[engineer(str_retype, default_value = "\"fa\".to_string()")]
    lang_code: Option<String>,
}

fn print_identity(ident: impl Into<Identity>) {
    let ident: Identity = ident.into();
    println!("{ident:#?}");
}

fn main() {
    let ident = Identity::new(1, "immmdreza", "Arash").last_name("Tofani");

    print_identity(ident);
    // Identity {
    //     id: 1,
    //     username: "immmdreza",
    //     first_name: "Arash",
    //     last_name: Some(
    //         "Tofani",
    //     ),
    //     lang_code: Some(
    //         "fa",
    //     ),
    // }
}
```

🧀
