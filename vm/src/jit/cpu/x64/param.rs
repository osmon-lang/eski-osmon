pub static OFFSTET: isize = 16;

pub fn next_param_offset(param_offset: isize) -> isize {
    param_offset + 8
}
