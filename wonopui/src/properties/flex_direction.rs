#[derive(PartialEq, Clone)]
pub enum FlexDirection {
    Row,
    Column,
}

impl ToString for FlexDirection {
    fn to_string(&self) -> String {
        match self {
            FlexDirection::Row => "flex-row".to_string(),
            FlexDirection::Column => "flex-col".to_string(),
        }
    }
}
