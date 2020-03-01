use crate::{GQLQueryError, GQLScalar, Result, Value};
use uuid::Uuid;

impl GQLScalar for Uuid {
    fn type_name() -> &'static str {
        "UUID"
    }

    fn parse(value: Value) -> Result<Self> {
        match value {
            Value::String(s) => Ok(Uuid::parse_str(&s)?),
            _ => {
                return Err(GQLQueryError::ExpectedType {
                    expect: Self::type_name().to_string(),
                    actual: value,
                }
                .into())
            }
        }
    }

    fn into_json(self) -> Result<serde_json::Value> {
        Ok(self.to_string().into())
    }
}
