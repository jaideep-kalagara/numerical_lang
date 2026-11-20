#[derive(Debug, Clone, Copy, PartialEq, strum::EnumString, strum::VariantNames, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum Keyword {
    Let,
    If,
    Func,
}
