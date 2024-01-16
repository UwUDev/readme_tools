use std::collections::HashMap;
use std::io::Write;

fn gradient_separator(start_color: (u8, u8, u8), end_color: (u8, u8, u8), length: usize) -> Result<String, &'static str> {
    if length < 2 {
        return Err("Length must be greater than 1");
    }

    if length > 49 {
        return Err("Length must be lower than 50");
    }

    println!("length: {}", length);

    let mut separator = String::from(r"$\textsf{");

    for i in 0..length {
        let ratio = i as f32 / (length - 1) as f32;
        let color = (
            (start_color.0 as f32 + (end_color.0 as f32 - start_color.0 as f32) * ratio) as u8,
            (start_color.1 as f32 + (end_color.1 as f32 - start_color.1 as f32) * ratio) as u8,
            (start_color.2 as f32 + (end_color.2 as f32 - start_color.2 as f32) * ratio) as u8,
        );

        let color_hex = format!(
            "#{:02X}{:02X}{:02X}",
            color.0, color.1, color.2
        );

        separator.push_str(&format!(r"\color{{{}}}{{█}}", color_hex));
    }

    separator.push('}');
    separator.push('$');


    Ok(separator)
}

fn gradient_text(
    start_color: (u8, u8, u8),
    end_color: (u8, u8, u8),
    text: &str,
) -> Result<String, &'static str> {
    let mut separator = String::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    let total_length = words.iter().map(|word| word.len()).sum::<usize>() + words.len() - 1;
    let mut current_length = 0;

    for word in words.clone() {
        let word_length = word.len();
        if word_length > 49 {
            return Err("Word too long");
        }
    }

    for word in words {
        separator.push_str(r"$\textsf{");

        for char in word.chars() {
            let ratio = current_length as f32 / (total_length - 1) as f32;
            let color = (
                (start_color.0 as f32 + (end_color.0 as f32 - start_color.0 as f32) * ratio) as u8,
                (start_color.1 as f32 + (end_color.1 as f32 - start_color.1 as f32) * ratio) as u8,
                (start_color.2 as f32 + (end_color.2 as f32 - start_color.2 as f32) * ratio) as u8,
            );

            let color_hex = format!(
                "#{:02X}{:02X}{:02X}",
                color.0, color.1, color.2
            );

            separator.push_str(&format!(r"\color{{{}}}{{{}}}", color_hex, char));
            current_length += 1;
        }

        separator.push_str("}$ ");
        // ajoute un espace dans le comptage de longueur pour le prochain mot
        current_length += 1;
    }

    Ok(separator.trim_end().to_string())
}

fn gradient_text_svg(
    start_color: (u8, u8, u8),
    end_color: (u8, u8, u8),
    scale_factor: f32,
    text: &str,
) -> Result<String, &'static str> {
    let mut map_length = HashMap::new();

    map_length.insert("a", 7.3);
    map_length.insert("b", 8.6);
    map_length.insert("c", 6.7);
    map_length.insert("d", 8.1);
    map_length.insert("e", 7.6);
    map_length.insert("f", 4.7);
    map_length.insert("g", 8.2);
    map_length.insert("h", 8.7);
    map_length.insert("i", 4.0);
    map_length.insert("j", 4.0);
    map_length.insert("k", 7.5);
    map_length.insert("l", 3.0);
    map_length.insert("m", 12.7);
    map_length.insert("n", 8.4);
    map_length.insert("o", 8.4);
    map_length.insert("p", 8.7);
    map_length.insert("q", 8.2);
    map_length.insert("r", 5.5);
    map_length.insert("s", 6.7);
    map_length.insert("t", 4.7);
    map_length.insert("u", 8.4);
    map_length.insert("v", 7.3);
    map_length.insert("w", 11.5);
    map_length.insert("x", 7.3);
    map_length.insert("y", 7.3);
    map_length.insert("z", 6.7);


    map_length.insert("A", 10.0);
    map_length.insert("B", 8.7);
    map_length.insert("C", 9.7);
    map_length.insert("D", 10.3);
    map_length.insert("E", 8.2);
    map_length.insert("F", 8.2);
    map_length.insert("G", 10.0);
    map_length.insert("H", 10.0);
    map_length.insert("I", 4.0);
    map_length.insert("J", 4.0);
    map_length.insert("K", 9.0);
    map_length.insert("L", 7.1);
    map_length.insert("M", 12.9);
    map_length.insert("N", 11.0);
    map_length.insert("O", 10.3);
    map_length.insert("P", 8.7);
    map_length.insert("Q", 11.7);
    map_length.insert("R", 9.0);
    map_length.insert("S", 8.2);
    map_length.insert("T", 8.2);
    map_length.insert("U", 10.0);
    map_length.insert("V", 9.0);
    map_length.insert("W", 13.8);
    map_length.insert("X", 9.0);
    map_length.insert("Y", 9.0);
    map_length.insert("Z", 8.2);


    map_length.insert("1", 6.3);
    map_length.insert("2", 7.3);
    map_length.insert("3", 7.3);
    map_length.insert("4", 7.3);
    map_length.insert("5", 7.3);
    map_length.insert("6", 7.3);
    map_length.insert("7", 7.3);
    map_length.insert("8", 7.3);
    map_length.insert("9", 7.3);
    map_length.insert("0", 7.3);

    map_length.insert(" ", 5.3);
    map_length.insert("!", 3.4);
    map_length.insert("?", 7.3);
    map_length.insert(".", 3.4);
    map_length.insert("'", 2.0);
    map_length.insert(",", 3.4);
    map_length.insert(";", 3.4);
    map_length.insert(":", 3.4);
    map_length.insert("(", 4.0);
    map_length.insert(")", 4.0);
    map_length.insert("[", 4.0);
    map_length.insert("]", 4.0);



    let mut current_x = 0.0;
    let mut current_y = scale_factor * 16.0;

    let mut anticipated_height = 0.0;

    for char in text.chars() {
        if char == '\n' {
            current_x = 0.0;
            anticipated_height += scale_factor * 16.0;
        } else {
            current_x += map_length.get(char.to_string().as_str()).unwrap() * scale_factor;
            if current_x > 500.0 - scale_factor * 16.0 {
                current_x = 0.0;
                anticipated_height += scale_factor * 16.0;
            }
        }
    }

    let mut current_x = 0.0;

    let mut svg = String::new();
    svg.push_str(r#"<?xml version="1.0" encoding="utf-8"?><svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="500" height=""#);

    svg.push_str((anticipated_height + (scale_factor * 16.0)*1.5).to_string().as_str());
    svg.push_str(r#"" font-size=""#);

    svg.push_str((16.0 * scale_factor).to_string().as_str());

    svg.push_str(r#"px" font-family="-apple-system,BlinkMacSystemFont,'Segoe UI','Noto Sans',Helvetica,Arial,sans-serif,'Apple Color Emoji','Segoe UI Emoji'">"#);

    let ratio_max = match anticipated_height / 500.0 {
        ratio if ratio > 1.0 => ratio,
        _ => 1.0,
    };

    for char in text.chars() {
        let ratio = current_x / (500.0 * ratio_max);
        let color = (
            (start_color.0 as f32 + (end_color.0 as f32 - start_color.0 as f32) * ratio) as u8,
            (start_color.1 as f32 + (end_color.1 as f32 - start_color.1 as f32) * ratio) as u8,
            (start_color.2 as f32 + (end_color.2 as f32 - start_color.2 as f32) * ratio) as u8,
        );

        let color_hex = format!(
            "#{:02X}{:02X}{:02X}",
            color.0, color.1, color.2
        );

        svg.push_str(&format!(r#"<text x="{}" y="{}" fill="{}">{}</text>"#, current_x, current_y, color_hex, char));
        current_x += map_length.get(char.to_string().as_str()).unwrap() * scale_factor;
        if current_x > 500.0 - scale_factor * 16.0 {
            current_x = 0.0;
            current_y += scale_factor * 16.0;
        }
    }

    svg.push_str(r#"</svg>"#);

    Ok(svg)
}

fn convert_hex_color_to_rgb(hex_color: &str) -> Result<(u8, u8, u8), &'static str> {
    if hex_color.len() != 7 {
        return Err("Hex color must be 7 characters long");
    }

    if !hex_color.starts_with('#') {
        return Err("Hex color must start with #");
    }

    let r = u8::from_str_radix(&hex_color[1..3], 16);
    let g = u8::from_str_radix(&hex_color[3..5], 16);
    let b = u8::from_str_radix(&hex_color[5..7], 16);

    match (r, g, b) {
        (Ok(r), Ok(g), Ok(b)) => Ok((r, g, b)),
        _ => Err("Hex color must be in the form #RRGGBB"),
    }
}

fn main() {
    let start_color = convert_hex_color_to_rgb("#999999").unwrap();
    let end_color = convert_hex_color_to_rgb("#111111").unwrap();

    let separator = gradient_separator(start_color, end_color, 49);
    println!("{}\n", separator.unwrap());

    let start_color = convert_hex_color_to_rgb("#8E31F7").unwrap();
    let end_color = convert_hex_color_to_rgb("#FC03DF").unwrap();

    let separator = gradient_text(start_color, end_color, "Who said you cant color a README.md file ?");
    let file = std::fs::File::create("gradient.md").unwrap();
    let mut file = std::io::BufWriter::new(file);
    file.write_all(separator.unwrap().as_bytes()).unwrap();

    let start_color = convert_hex_color_to_rgb("#22c1c3").unwrap();
    let end_color = convert_hex_color_to_rgb("#fdbb2d").unwrap();
    let separator = gradient_text(start_color, end_color, "Yes this is a README.md file with a gradient text, i'm just abusing the fact that you can use latex in a markdown file.");
    let file = std::fs::File::create("gradient2.md").unwrap();
    let mut file = std::io::BufWriter::new(file);
    file.write_all(separator.unwrap().as_bytes()).unwrap();

    let start_color = convert_hex_color_to_rgb("#3f5efb").unwrap();
    let end_color = convert_hex_color_to_rgb("#fc466b").unwrap();
    let separator = gradient_text_svg(start_color, end_color, 1.0,"You can also generate a svg file with a gradient text, but you need to     specify the scale factor, the text and the colors. (1.0 is github default fontsize)");
    let file = std::fs::File::create("gradient1.svg").unwrap();
    let mut file = std::io::BufWriter::new(file);
    file.write_all(separator.unwrap().as_bytes()).unwrap();

    let start_color = convert_hex_color_to_rgb("#eeaeca").unwrap();
    let end_color = convert_hex_color_to_rgb("#94bbe9").unwrap();
    let separator = gradient_text_svg(start_color, end_color, 2.0,"And make them bigger");
    let file = std::fs::File::create("gradient2.svg").unwrap();
    let mut file = std::io::BufWriter::new(file);
    file.write_all(separator.unwrap().as_bytes()).unwrap();
}