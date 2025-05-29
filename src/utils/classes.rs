// Utility module for CSS class handling

use yew::Classes;

/// Combines multiple class strings into a single Classes instance
pub fn combine_classes(classes: &[&str]) -> Classes {
    let mut result = Classes::new();
    for class in classes {
        result.push(*class);
    }
    result
}

/// Conditionally add a class
pub fn conditional_class(condition: bool, class: &str) -> Classes {
    if condition {
        classes!(class)
    } else {
        Classes::new()
    }
}