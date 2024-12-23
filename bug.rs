fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 4;
    }
    println!( "Value at index 0: {:?}", v[0]);
}