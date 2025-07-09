# ToSqlEnum

**ToSqlEnum** is a generic trait used to serialize your enums for use in SQL table definitions


## Example
You must `use` the trait first, then it can be used on any compliant enum

The following code will serialize `MyEnum` into
```
ENUM('foo','bar')
```

```rust
use strum_macros::{Display, EnumIter};
use to_sql_enum::ToSqlEnum;

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
```
