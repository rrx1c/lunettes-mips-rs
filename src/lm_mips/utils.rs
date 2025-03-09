pub fn num_to_str(str: &mut [char], buffer_size: isize, mut num: u64) -> bool{
    let digit_num: usize;
    let mut i: usize;

    if buffer_size < 5 ||
    num < 0x10000 && buffer_size < 7 ||
    num < 0x100000000 && buffer_size < 11 ||
    buffer_size < 19{
        return false
    }

    str[0] = '0';
    str[1] = 'x';

    if num < 0x100{
        digit_num = 2;
    }
    else if num < 0x10000{
        digit_num = 4;
    }
    else if num < 0x100000000{
        digit_num = 8;
    }
    else {
        digit_num = 16;
    }

    i = digit_num;
    while i > 0{
        match num & 0xf{
            0..=9 => str[i + 1] = ((num & 0xf) + 0x30) as u8 as char,
            _ => str[i + 1] = ((num & 0xf) + 0x37) as u8 as char
        }
        num >>= 4;
        i-=1;
    }

    true
}