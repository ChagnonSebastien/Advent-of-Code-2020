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