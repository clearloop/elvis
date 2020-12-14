use elvis_core::Class;

/// Parse class to web string
pub fn parse_class(class: &Class) -> &'static str {
    match class {
        Class::Center => "center",
        Class::Flex => "flex",
        Class::Col => "col",
        Class::Row => "row",
        Class::Empty => "",
    }
}
