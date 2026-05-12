#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalizationError {
    CarriageReturn { byte_index: usize },
    HorizontalTab { byte_index: usize },
    NonAsciiControl { byte_index: usize, byte: u8 },
}

pub fn canonical_lines(input: &str) -> Result<Vec<String>, CanonicalizationError> {
    let bytes = input.as_bytes();
    for (index, byte) in bytes.iter().enumerate() {
        match *byte {
            b'\r' => return Err(CanonicalizationError::CarriageReturn { byte_index: index }),
            b'\t' => return Err(CanonicalizationError::HorizontalTab { byte_index: index }),
            0x00..=0x08 | 0x0b..=0x1f | 0x7f => {
                return Err(CanonicalizationError::NonAsciiControl {
                    byte_index: index,
                    byte: *byte,
                })
            }
            _ => {}
        }
    }

    let normalized = if let Some(stripped) = input.strip_suffix('\n') {
        stripped
    } else {
        input
    };

    Ok(normalized
        .split('\n')
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect())
}

pub fn canonical_surface_text(input: &str) -> Result<String, CanonicalizationError> {
    let mut lines = canonical_lines(input)?;
    if lines.is_empty() {
        return Ok(String::new());
    }

    let header = lines.remove(0);
    lines.sort();
    lines.dedup();

    let mut output = String::new();
    output.push_str(&header);
    output.push('\n');
    for line in lines {
        output.push_str(&line);
        output.push('\n');
    }
    Ok(output)
}
