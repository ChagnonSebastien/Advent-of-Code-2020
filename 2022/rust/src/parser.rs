pub(crate) fn read_unsigned_int(buffer: &[u8], offset: &mut usize) -> Result<usize, &'static str> {
    if buffer[*offset] < '0' as u8 || buffer[*offset] > '9' as u8 {
        return Err("offset is not pointing at an unsigned integer");
    }

    let mut value: usize = 0;
    loop {
        value = value * 10 + (buffer[*offset] as usize - '0' as usize);
        *offset += 1;
        if buffer[*offset] < '0' as u8 || buffer[*offset] > '9' as u8 {
            break;
        }
    }
    return Ok(value);
}

pub(crate) fn read_word<'a>(buffer: &'a [u8], offset: &mut usize) -> Result<&'a[u8], &'static str> {
    if buffer[*offset] < 'a' as u8 || buffer[*offset] > 'z' as u8 {
        return Err("offset is not pointing at a string");
    }

    let starting_offset = *offset;
    loop {
        *offset += 1;
        if buffer[*offset] < 'a' as u8 || buffer[*offset] > 'z' as u8 {
            break;
        }
    }
    //println!("reading '{}'", String::from_utf8(Vec::from(&buffer[starting_offset..*offset])).unwrap());
    return Ok(&buffer[starting_offset..*offset]);
}

pub(crate) fn move_to_next_line(buffer: &[u8], offset: &mut usize) {
    //let starting_offset = *offset;
    //print!("moving offset from {}", *offset);
    while buffer[*offset] != '\n' as u8 {
        *offset += 1;
    }

    // At the line break, need to move one more
    *offset += 1;
    //println!(" to {}, skipping over '{}'", *offset, String::from_utf8(Vec::from(&buffer[starting_offset..*offset])).unwrap())
}