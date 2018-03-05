/// Module for managing specification definitions.
mod data;

/// Verifies that a serde_yaml specification is valid for conversion.
/// Returns an error message describing any failures.
pub fn verify(yaml: &HashMap<String, Value>) -> Option<String> {
    // todo: Verify configuration.
    // Ensure no duplicates in "variables" section.
    // Ensure no duplicates in "struct" section.
    None
}
