use dobby_rs::{hook, Address};

#[inline(never)]
#[no_mangle]
extern "C" fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[inline(never)]
#[no_mangle]
extern "C" fn sub(a: u64, b: u64) -> u64 {
    a - b
}

#[test]
fn test_hook() {
    let addr = add as usize as Address;
    let replace = sub as usize as Address;

    let c = add(7, 5);
    assert_eq!(c, 7 + 5);

    unsafe {
        println!("addr={:x} replace={:x}", addr as usize, replace as usize);
        let origin = hook(addr, replace).unwrap();
        println!("origin={:x}", origin as usize);
    }

    let c = add(7, 5);
    assert_eq!(c, 7 - 5);
}
