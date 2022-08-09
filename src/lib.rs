use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
enum Example {
    ChoiceA(ChoiceAStruct),
    ChoiceB(String),
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct ChoiceAStruct {
    field_a: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_to_json_correctly() {
        insta::assert_json_snapshot!(vec![
            Example::ChoiceA(ChoiceAStruct {
                field_a: "a".to_string()
            }),
            Example::ChoiceB("b".to_string()),
        ]);
    }
}
