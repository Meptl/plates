/// Converts snake_case to CamelCase
fn snake_to_camel_case(s: &str) -> String {
    s.split('_').map(|section| {
        let mut r = String::with_capacity(section.len());
        for (i, c) in section.chars().enumerate() {
            match i {
                0 => r.extend(c.to_uppercase()),
                _ => r.extend(c.to_lowercase()),
            }
        }
        r
    }).collect()
}

/// Converts snake_case to lowerCamelCase
fn snake_to_lower_camel_case(s: &str) -> String {
    s.split('_').enumerate().map(|(j, section)| {
        let mut r = String::with_capacity(section.len());
        for (i, c) in section.chars().enumerate() {
            match i {
                0 => if j != 0 { r.extend(c.to_uppercase()) } else { r.extend(c.to_lowercase()) }
                _ => r.extend(c.to_lowercase()),
            }
        }
        r
    }).collect()
}

