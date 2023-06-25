// see https://users.rust-lang.org/t/solved-serde-deserialize-with-for-option-s/12749/2
pub mod timeser {
    use serde::{Deserialize, Deserializer, Serialize};
    use time::OffsetDateTime;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
        where
            D: Deserializer<'de>,
    {
        Option::<WrappedOffsetDateTime>::deserialize(deserializer)
            .map(|opt_wrapped: Option<WrappedOffsetDateTime>| {
                opt_wrapped.map(|wrapped: WrappedOffsetDateTime| wrapped.value)
            })
    }

    pub fn serialize<S>(date: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        date.map(|date| WrappedOffsetDateTime { value: date.clone() }).serialize(serializer)
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct WrappedOffsetDateTime {
        #[serde(with = "time::serde::rfc3339")]
        pub value: OffsetDateTime,
    }
}