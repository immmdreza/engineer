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
    #[derive(Debug)]
    enum Gender {
        Male,
        Female,
    }

    #[allow(dead_code)]
    #[derive(Debug, Engineer)]
    #[engineer(new, str_retype)]
    struct Model {
        id: usize,
        first_name: String,
        last_name: Option<String>,
        gender: Option<Gender>,
        #[engineer(default_value = r#"String::from("fa")"#)]
        lang_code: Option<String>,
    }

    #[test]
    fn basic_test() {
        let model = Model::new(1, "Arash").done();

        assert_eq!(model.id, 1);
        assert_eq!(model.first_name, "Arash");
        assert_eq!(model.last_name, None);
        assert!(matches!(model.gender, None));
        assert_eq!(model.lang_code, Some("fa".to_string()));
    }

    #[test]
    fn basic_test_2() {
        let model = Model::new(1, "Arash")
            .last_name("Tofani")
            .gender(Gender::Male)
            .lang_code("en")
            .done();

        assert_eq!(model.id, 1);
        assert_eq!(model.first_name, "Arash");
        assert_eq!(model.last_name, Some("Tofani".to_string()));
        assert!(matches!(model.gender, Some(Gender::Male)));
        assert_eq!(model.lang_code, Some("en".to_string()));
    }
}
