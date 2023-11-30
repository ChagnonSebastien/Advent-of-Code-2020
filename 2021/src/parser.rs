pub(crate) fn read_unsigned_int(buffer: &[u8], offset: &mut usize) -> usize {
    if buffer[*offset] < '0' as u8 || buffer[*offset] > '9' as u8 {
        panic!("offset is not pointing at an unsigned integer");
    }

    let mut value: usize = 0;
    loop {
        value = value * 10 + (buffer[*offset] as usize - '0' as usize);
        *offset += 1;
        if buffer[*offset] < '0' as u8 || buffer[*offset] > '9' as u8 {
            break;
        }
    }
    return value;
}

#[allow(dead_code)]
pub(crate) fn read_signed_int(buffer: &[u8], offset: &mut usize) -> isize {
    let sign = match buffer[*offset] as char {
        '-' => {
            *offset += 1;
            -1
        },
        _ => 1
    };

    return read_unsigned_int(buffer, offset) as isize * sign;
}

pub(crate) fn skip_word(buffer: &[u8], offset: &mut usize) {
    while buffer[*offset] >= 'a' as u8 && buffer[*offset] <= 'z' as u8 {
        *offset += 1;
    }
}

#[allow(dead_code)]
pub(crate) fn read_word<'a>(buffer: &'a [u8], offset: &mut usize) -> Result<&'a[u8], &'static str> {
    let starting_offset = *offset;
    skip_word(buffer, offset);

    if starting_offset == *offset {
        return Err("offset is not pointing at a string");
    }

    return Ok(&buffer[starting_offset..*offset]);
}

#[allow(dead_code)]
pub(crate) fn move_to_next_line(buffer: &[u8], offset: &mut usize) {
    while buffer[*offset] != '\n' as u8 {
        *offset += 1;
    }

    // At the line break, need to move one more
    *offset += 1;
}