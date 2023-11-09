fn as_bytes<T>(o: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(o as *const _ as *const u8, std::mem::size_of::<T>()) }
}

fn main() {
    println!("{:?}", as_bytes(&1i8));
    println!("{:?}", as_bytes(&2i16));
    println!("{:?}", as_bytes(&3i32));
    println!("{:?}", as_bytes(&(4i64 + 5 * 256 + 6 * 256 * 256)));
    println!("{:?}", as_bytes(&'A'));
    println!("{:?}", as_bytes(&true));
    println!("{:?}", as_bytes(&&1i8));
}

