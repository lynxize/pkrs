pub fn get_input(prompt: &str) -> String {
    let mut inp = String::new();
    println!("{}", prompt);
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");
    inp
}

// see https://users.rust-lang.org/t/solved-serde-deserialize-with-for-option-s/12749/2
pub mod timeser {
    use time::OffsetDateTime;
    use serde::{Deserialize, Deserializer, Serialize};

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
    struct WrappedOffsetDateTime{
        #[serde(with="time::serde::rfc3339")]
        pub value: OffsetDateTime
    }
}