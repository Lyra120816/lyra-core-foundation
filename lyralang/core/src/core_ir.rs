#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CoreIrDescriptor {
    pub id: &'static str,
    pub medium: &'static str,
    pub version: &'static str,
    pub canonical_header: &'static str,
    pub extension: &'static str,
    pub encoding_law: &'static str,
    pub canonicalization_law: &'static str,
    pub round_trip_law: &'static str,
    pub upgrade_law: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreIrCodecError {
    EmptyField {
        field: &'static str,
    },
    InvalidByte {
        field: &'static str,
        byte_index: usize,
        byte: u8,
    },
    FieldTooLong {
        field: &'static str,
        length: usize,
    },
    InvalidMagic,
    InvalidVersion {
        major: u16,
        minor: u16,
    },
    TruncatedFrame,
    TrailingBytes {
        remaining: usize,
    },
    InvalidHexLength,
    InvalidHexByte {
        index: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrTextFrame {
    pub kind: String,
    pub atom: String,
    pub payload: String,
}

pub const LYRA_CORE_IR_MAJOR: u16 = 1;
pub const LYRA_CORE_IR_MINOR: u16 = 0;
pub const LYRA_CORE_IR_BINARY_MAGIC: &[u8; 8] = b"LYRAIR01";
pub const LYRA_CORE_IR_TEXT_HEADER: &str = "LYRA-CORE-IR-TEXT v1";

pub const LYRALANG_CORE_IR_DESCRIPTORS: &[CoreIrDescriptor] = &[
    CoreIrDescriptor {
        id: "text_ir",
        medium: "text",
        version: "ir_v1",
        canonical_header: "LYRA_CORE_IR_TEXT_V1",
        extension: "lyrair",
        encoding_law: "utf8_canonical_lines",
        canonicalization_law: "sorted_key_value_lines",
        round_trip_law: "text_to_binary_to_text_identity",
        upgrade_law: "explicit_version_edge_only",
    },
    CoreIrDescriptor {
        id: "binary_ir",
        medium: "binary",
        version: "ir_v1",
        canonical_header: "LYRAIR01",
        extension: "lyrairb",
        encoding_law: "length_prefixed_big_endian",
        canonicalization_law: "canonical_binary_frame",
        round_trip_law: "binary_to_text_to_binary_identity",
        upgrade_law: "explicit_version_edge_only",
    },
];

pub fn core_ir_ids() -> Vec<&'static str> {
    LYRALANG_CORE_IR_DESCRIPTORS
        .iter()
        .map(|form| form.id)
        .collect()
}

pub fn core_ir_descriptor(id: &str) -> Option<CoreIrDescriptor> {
    LYRALANG_CORE_IR_DESCRIPTORS
        .iter()
        .copied()
        .find(|form| form.id == id)
}

pub fn canonical_core_ir_signature(descriptor: CoreIrDescriptor) -> String {
    format!(
        "ir_form:{}|medium:{}|version:{}|header:{}|extension:{}|encoding:{}|canonicalization:{}|round_trip:{}|upgrade:{}",
        descriptor.id,
        descriptor.medium,
        descriptor.version,
        descriptor.canonical_header,
        descriptor.extension,
        descriptor.encoding_law,
        descriptor.canonicalization_law,
        descriptor.round_trip_law,
        descriptor.upgrade_law,
    )
}

pub fn canonical_core_ir_registry_signature() -> String {
    let mut signatures: Vec<String> = LYRALANG_CORE_IR_DESCRIPTORS
        .iter()
        .copied()
        .map(canonical_core_ir_signature)
        .collect();
    signatures.sort();
    signatures.join("\n")
}

pub fn canonical_text_ir(
    kind: &str,
    atom: &str,
    payload: &str,
) -> Result<String, CoreIrCodecError> {
    validate_ascii_field("kind", kind)?;
    validate_ascii_field("atom", atom)?;
    validate_ascii_field("payload", payload)?;
    let mut lines = vec![
        format!("atom={atom}"),
        format!("kind={kind}"),
        format!("payload={payload}"),
        format!("version={}.{}", LYRA_CORE_IR_MAJOR, LYRA_CORE_IR_MINOR),
    ];
    lines.sort();
    let mut output = String::new();
    output.push_str(LYRA_CORE_IR_TEXT_HEADER);
    output.push('\n');
    for line in lines {
        output.push_str(&line);
        output.push('\n');
    }
    Ok(output)
}

pub fn parse_canonical_text_ir(input: &str) -> Result<CoreIrTextFrame, CoreIrCodecError> {
    let mut lines: Vec<&str> = input
        .strip_suffix('\n')
        .unwrap_or(input)
        .split('\n')
        .collect();
    if lines.is_empty() || lines.remove(0) != LYRA_CORE_IR_TEXT_HEADER {
        return Err(CoreIrCodecError::InvalidMagic);
    }
    let mut kind = None;
    let mut atom = None;
    let mut payload = None;
    let mut version = None;
    for line in lines {
        let (key, value) = line
            .split_once('=')
            .ok_or(CoreIrCodecError::TruncatedFrame)?;
        match key {
            "atom" => atom = Some(value.to_string()),
            "kind" => kind = Some(value.to_string()),
            "payload" => payload = Some(value.to_string()),
            "version" => version = Some(value.to_string()),
            _ => {
                return Err(CoreIrCodecError::TrailingBytes {
                    remaining: line.len(),
                })
            }
        }
    }
    if version.as_deref() != Some("1.0") {
        return Err(CoreIrCodecError::InvalidVersion { major: 0, minor: 0 });
    }
    let kind = kind.ok_or(CoreIrCodecError::TruncatedFrame)?;
    let atom = atom.ok_or(CoreIrCodecError::TruncatedFrame)?;
    let payload = payload.ok_or(CoreIrCodecError::TruncatedFrame)?;
    validate_ascii_field("kind", &kind)?;
    validate_ascii_field("atom", &atom)?;
    validate_ascii_field("payload", &payload)?;
    Ok(CoreIrTextFrame {
        kind,
        atom,
        payload,
    })
}

pub fn encode_binary_ir_frame(
    kind: &str,
    atom: &str,
    payload: &str,
) -> Result<Vec<u8>, CoreIrCodecError> {
    validate_ascii_field("kind", kind)?;
    validate_ascii_field("atom", atom)?;
    validate_ascii_field("payload", payload)?;
    let mut output = Vec::new();
    output.extend_from_slice(LYRA_CORE_IR_BINARY_MAGIC);
    output.extend_from_slice(&LYRA_CORE_IR_MAJOR.to_be_bytes());
    output.extend_from_slice(&LYRA_CORE_IR_MINOR.to_be_bytes());
    push_len_field(&mut output, "kind", kind)?;
    push_len_field(&mut output, "atom", atom)?;
    push_len_field(&mut output, "payload", payload)?;
    Ok(output)
}

pub fn decode_binary_ir_frame(bytes: &[u8]) -> Result<CoreIrTextFrame, CoreIrCodecError> {
    if bytes.len() < LYRA_CORE_IR_BINARY_MAGIC.len() + 4 {
        return Err(CoreIrCodecError::TruncatedFrame);
    }
    if &bytes[..LYRA_CORE_IR_BINARY_MAGIC.len()] != LYRA_CORE_IR_BINARY_MAGIC {
        return Err(CoreIrCodecError::InvalidMagic);
    }
    let mut cursor = LYRA_CORE_IR_BINARY_MAGIC.len();
    let major = read_u16(bytes, &mut cursor)?;
    let minor = read_u16(bytes, &mut cursor)?;
    if major != LYRA_CORE_IR_MAJOR || minor != LYRA_CORE_IR_MINOR {
        return Err(CoreIrCodecError::InvalidVersion { major, minor });
    }
    let kind = read_len_field(bytes, &mut cursor, "kind")?;
    let atom = read_len_field(bytes, &mut cursor, "atom")?;
    let payload = read_len_field(bytes, &mut cursor, "payload")?;
    if cursor != bytes.len() {
        return Err(CoreIrCodecError::TrailingBytes {
            remaining: bytes.len() - cursor,
        });
    }
    Ok(CoreIrTextFrame {
        kind,
        atom,
        payload,
    })
}

pub fn text_ir_to_binary(input: &str) -> Result<Vec<u8>, CoreIrCodecError> {
    let frame = parse_canonical_text_ir(input)?;
    encode_binary_ir_frame(&frame.kind, &frame.atom, &frame.payload)
}

pub fn binary_ir_to_text(bytes: &[u8]) -> Result<String, CoreIrCodecError> {
    let frame = decode_binary_ir_frame(bytes)?;
    canonical_text_ir(&frame.kind, &frame.atom, &frame.payload)
}

pub fn round_trip_text_identity(input: &str) -> Result<bool, CoreIrCodecError> {
    let binary = text_ir_to_binary(input)?;
    let text = binary_ir_to_text(&binary)?;
    Ok(text == input)
}

pub fn round_trip_binary_identity(bytes: &[u8]) -> Result<bool, CoreIrCodecError> {
    let text = binary_ir_to_text(bytes)?;
    let binary = text_ir_to_binary(&text)?;
    Ok(binary == bytes)
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(nibble_to_hex(byte >> 4));
        output.push(nibble_to_hex(byte & 0x0f));
    }
    output
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, CoreIrCodecError> {
    if hex.len() % 2 != 0 {
        return Err(CoreIrCodecError::InvalidHexLength);
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    let chars: Vec<u8> = hex.as_bytes().to_vec();
    let mut index = 0usize;
    while index < chars.len() {
        let high = hex_value(chars[index]).ok_or(CoreIrCodecError::InvalidHexByte { index })?;
        let low = hex_value(chars[index + 1])
            .ok_or(CoreIrCodecError::InvalidHexByte { index: index + 1 })?;
        bytes.push((high << 4) | low);
        index += 2;
    }
    Ok(bytes)
}

fn validate_ascii_field(field: &'static str, value: &str) -> Result<(), CoreIrCodecError> {
    if value.is_empty() {
        return Err(CoreIrCodecError::EmptyField { field });
    }
    for (index, byte) in value.as_bytes().iter().copied().enumerate() {
        if !(byte.is_ascii_alphanumeric()
            || byte == b'_'
            || byte == b'.'
            || byte == b'-'
            || byte == b'/')
        {
            return Err(CoreIrCodecError::InvalidByte {
                field,
                byte_index: index,
                byte,
            });
        }
    }
    Ok(())
}

fn push_len_field(
    output: &mut Vec<u8>,
    field: &'static str,
    value: &str,
) -> Result<(), CoreIrCodecError> {
    if value.len() > u16::MAX as usize {
        return Err(CoreIrCodecError::FieldTooLong {
            field,
            length: value.len(),
        });
    }
    output.extend_from_slice(&(value.len() as u16).to_be_bytes());
    output.extend_from_slice(value.as_bytes());
    Ok(())
}

fn read_u16(bytes: &[u8], cursor: &mut usize) -> Result<u16, CoreIrCodecError> {
    if *cursor + 2 > bytes.len() {
        return Err(CoreIrCodecError::TruncatedFrame);
    }
    let value = u16::from_be_bytes([bytes[*cursor], bytes[*cursor + 1]]);
    *cursor += 2;
    Ok(value)
}

fn read_len_field(
    bytes: &[u8],
    cursor: &mut usize,
    field: &'static str,
) -> Result<String, CoreIrCodecError> {
    let length = read_u16(bytes, cursor)? as usize;
    if length == 0 {
        return Err(CoreIrCodecError::EmptyField { field });
    }
    if *cursor + length > bytes.len() {
        return Err(CoreIrCodecError::TruncatedFrame);
    }
    let slice = &bytes[*cursor..*cursor + length];
    *cursor += length;
    let value = String::from_utf8(slice.to_vec()).map_err(|_| CoreIrCodecError::InvalidByte {
        field,
        byte_index: 0,
        byte: 0xff,
    })?;
    validate_ascii_field(field, &value)?;
    Ok(value)
}

fn nibble_to_hex(nibble: u8) -> char {
    match nibble {
        0..=9 => (b'0' + nibble) as char,
        10..=15 => (b'a' + (nibble - 10)) as char,
        _ => '0',
    }
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
