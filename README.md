# Engineer

Engineer is a master builder based on `Optional`.

It just generates an Engineer (Builder) class for a data model.

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
#[engineer(engineer_name = "IdentityBuilder", builder_func = "builder")]
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
#[engineer(engineer_name = "IdentityBuilder", builder_func = "new")]
struct Identity {
    // ~~~ sniff ~~~
}

// ~~~ sniff ~~~

    let identity = Identity::new(0, "immmdreza".to_string())
    // ~~~ sniff ~~~
```

### Default value for Options

You can set a default value for option fields.

This value is used if you don't set any other for them.

```rust
// ~~~ sniff ~~~

#[derive(Engineer)]
#[engineer(engineer_name = "IdentityBuilder", builder_func = "new")]
struct Identity {
    // ~~~ sniff ~~~
    #[engineer(default = "\"fa\".to_string()")]
    lang_code: Option<String>,
}

// ~~~ sniff ~~~

    let identity = Identity::new(0, "immmdreza".to_string());

    identity.lang_code // Some("fa")
```

### Retype

You can change types requested in builder processes.

```rust
// ~~~ sniff ~~~

#[derive(Engineer)]
#[engineer(builder_func = "new")]
struct Identity {
    // ~~~ sniff ~~~
    #[engineer(retype(to = "&str", re = ".to_string()"))]
    //                      ^            ^
    //                      | Requested type in public.
    //                                   |
    //                                   | How we recover to original type.
    username: String,
    // ~~~ sniff ~~~
}

// ~~~ sniff ~~~

    let identity = Identity::new(0, "immmdreza"); // .to_string() is not needed.
    // ~~~ sniff ~~~
```

Final result

```rust
#[derive(Engineer)]
#[engineer(builder_func = "new")]
struct Identity {
    id: usize,

    #[engineer(retype(to = "&str", re = ".to_string()"))]
    username: String,

    #[engineer(retype(to = "&str", re = ".to_string()"))]
    first_name: Option<String>,

    #[engineer(retype(to = "&str", re = ".to_string()"))]
    last_name: Option<String>,

    #[engineer(
        retype(to = "&str", re = ".to_string()"),
        default = "\"fa\".to_string()"
    )]
    lang_code: Option<String>,
}
```

🧀
