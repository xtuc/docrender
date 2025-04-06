use super::gdoc::*;

pub(crate) fn from_text_style(style: &DocumentParagraphElementStyle) -> String {
    let mut out = "".to_owned();

    if style.bold {
        out += "font-weight:bold;"
    }

    if style.underline {
        out += "text-decoration:underline;"
    }

    if style.italic {
        out += "font-style:italic;"
    }

    if let Some(font_size) = &style.font_size {
        out += &format!("font-size:{}{};", font_size.magnitude, font_size.unit)
    }

    if let Some(foreground_color) = &style.foreground_color {
        out += &format!(
            "color: rgb({}%, {}%, {}%);",
            foreground_color.color.rgb_color.red.unwrap_or(0.0) * 100.0,
            foreground_color.color.rgb_color.green.unwrap_or(0.0) * 100.0,
            foreground_color.color.rgb_color.blue.unwrap_or(0.0) * 100.0
        );
    }

    if let Some(background_color) = &style.background_color {
        out += &format!(
            "background-color: rgb({}%, {}%, {}%);",
            background_color.color.rgb_color.red.unwrap_or(0.0) * 100.0,
            background_color.color.rgb_color.green.unwrap_or(0.0) * 100.0,
            background_color.color.rgb_color.blue.unwrap_or(0.0) * 100.0
        );
    }

    out
}
