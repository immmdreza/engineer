extern crate engineer_derive;

pub use engineer_derive::*;

pub trait Builder<T>
where
    T: Engineer,
    T::Builder: Builder<T>,
{
    fn done(self) -> T;
}

pub trait Engineer
where
    Self: Sized,
    Self::Builder: Builder<Self>,
{
    const NORMAL_FIELDS: usize;
    const OPTIONAL_FIELDS: usize;

    type Builder;
    type Params;

    fn builder(required: Self::Params) -> Self::Builder;

    fn build(required: Self::Params) -> Self {
        Self::builder(required).done()
    }

    fn build_default() -> Self
    where
        Self::Params: Default,
    {
        Self::build(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    #[derive(Debug, Default)]
    enum Gender {
        Male,
        Female,

        #[default]
        NotSaying,
    }

    #[allow(dead_code)]
    #[derive(Debug, Engineer)]
    #[engineer(new, str_retype)]
    struct User {
        id: usize,          // No default.
        first_name: String, // No default.
        // Default on none option.
        #[engineer(default)]
        gender: Gender, // Default should be Default::default() (Gender::NotSaying).
        // Default value on none option.
        #[engineer(default_value = r#"String::from("fa")"#)]
        lang_code: String, // Default should be String("fa").
        last_name: Option<String>, // Default should be None.
        // Default on option.
        #[engineer(default)] // Educated, not educated, no data
        is_educated: Option<bool>, // Default should be Some(Default::default()) (Some(false)).
        // Default value on option.
        #[engineer(default_value = "true")]
        is_human: Option<bool>, // Default should be Some(true).
    }

    #[test]
    fn basic_test() {
        let arash = User::new(1, "Arash").done();

        assert_eq!(arash.id, 1);
        assert_eq!(arash.first_name, "Arash");
        assert_eq!(arash.last_name, None);
        assert!(matches!(arash.gender, Gender::NotSaying));
        assert_eq!(arash.lang_code, "fa".to_string());
        assert!(matches!(arash.is_educated, Some(false)));
        assert!(matches!(arash.is_human, Some(true)));
    }

    #[test]
    fn basic_test_2() {
        let model = User::new(1, "Monkey")
            .last_name("Donkey")
            .gender(Gender::NotSaying)
            .lang_code("en")
            .is_human(false)
            .is_educated(true)
            .done();

        assert_eq!(model.id, 1);
        assert_eq!(model.first_name, "Monkey");
        assert_eq!(model.last_name, Some("Donkey".to_string()));
        assert!(matches!(model.gender, Gender::NotSaying));
        assert_eq!(model.lang_code, "en".to_string());
        assert!(matches!(model.is_educated, Some(true)));
        assert!(matches!(model.is_human, Some(false)));
    }
}
