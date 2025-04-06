use super::gdoc;

pub(crate) fn prefix(n: usize, properties: &gdoc::DocumentListNestingLevel) -> String {
    let prefix = if let Some(symbol) = &properties.glyph_symbol {
        if symbol == "-" {
            // Instead of using `-`, we use the default CSS
            // styling for <li>.
            "".to_owned()
        } else {
            format!("{symbol} ")
        }
    } else if let Some(t) = &properties.glyph_type {
        glyph_type_to_str(t, n)
    } else {
        "".to_owned()
    };

    prefix
}

fn glyph_type_to_str(t: &str, n: usize) -> String {
    match t {
        "ROMAN" => "I ".to_owned(),
        "DECIMAL" => format!("{}. ", n + 1),
        "ALPHA" => match n {
            0 => "a. ".to_owned(),
            1 => "b. ".to_owned(),
            2 => "c. ".to_owned(),
            3 => "d. ".to_owned(),
            _ => "".to_owned(),
        },
        "UPPER_ALPHA" => glyph_type_to_str("ALPHA", n).to_uppercase(),
        _ => "".to_owned(),
    }
}
