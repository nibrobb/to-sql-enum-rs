use strum_macros::{Display, EnumIter};
use to_sql_enum::*;

#[derive(Copy, Clone, EnumIter, Display)]
enum MyEnum {
    #[strum(serialize = "foo")]
    Foo,
    #[strum(serialize = "bar")]
    Bar,
}

fn main() {
    println!("{}", MyEnum::to_sql_enum())
}
