use strum::IntoEnumIterator;

pub trait ToSqlEnum {
    fn to_sql_enum() -> String;
}

impl<T> ToSqlEnum for T
where
    T: IntoEnumIterator + std::fmt::Display + Copy,
{
    fn to_sql_enum() -> String {
        let variants: Vec<String> = T::iter().map(|v| format!("'{v}'")).collect();
        format!("ENUM({})", variants.join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum_macros::{Display, EnumIter};

    #[derive(Copy, Clone, EnumIter, Display)]
    enum MockEnum {
        #[strum(serialize = "foo")]
        Foo,
        #[strum(serialize = "bar")]
        Bar,
    }

    #[test]
    fn format_as_expected() {
        let result = MockEnum::to_sql_enum();
        assert_eq!(result, "ENUM('foo','bar')")
    }
}
