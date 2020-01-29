#[repr(C)]
union Ambiguous {
    my_int: u32,
    addr: *const u32,
}

fn main() {
    let a = Ambiguous { my_int: 0x00 };
    println!("{}", *a.addr);
}
