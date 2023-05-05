pub fn process_optional_param(param: Option<Vec<String>>) -> Vec<String> {
    let mut processed_param = vec![];
    if let Some(populated_param) = param {
        processed_param = populated_param
    }
    processed_param
}
