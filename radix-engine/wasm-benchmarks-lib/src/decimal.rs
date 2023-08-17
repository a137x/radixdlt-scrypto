use radix_engine_common::math::*;

#[cfg(target_arch = "wasm32")]
extern "C" {
    pub fn decimal_add_native(a_ptr: *mut u8, b_ptr: *mut u8, c_ptr: *mut u8) -> u64;
    pub fn decimal_mul_native(a_ptr: *mut u8, b_ptr: *mut u8, c_ptr: *mut u8) -> u64;
    pub fn decimal_pow_native(a_ptr: *mut u8, b_ptr: *mut u8, c_ptr: *mut u8) -> u64;
}

#[cfg(not(target_arch = "wasm32"))]
pub unsafe fn decimal_add_native(_a_ptr: *mut u8, _b_ptr: *mut u8, _c_ptr: *mut u8) -> u64 {
    unreachable!()
}

#[cfg(not(target_arch = "wasm32"))]
pub unsafe fn decimal_mul_native(_a_ptr: *mut u8, _b_ptr: *mut u8, _c_ptr: *mut u8) -> u64 {
    unreachable!()
}

#[cfg(not(target_arch = "wasm32"))]
pub unsafe fn decimal_pow_native(_a_ptr: *mut u8, _b_ptr: *mut u8, _c_ptr: *mut u8) -> u64 {
    unreachable!()
}

#[no_mangle]
pub fn decimal_add() -> i64 {
    let x = Decimal::ONE;
    let y = Decimal::ONE;
    let z = x.safe_add(y).unwrap();
    z.is_positive().into()
}

#[no_mangle]
pub fn decimal_mul() -> i64 {
    let x = Decimal::ONE;
    let y = Decimal::ONE;
    let z = x.safe_mul(y).unwrap();
    z.is_positive().into()
}

#[no_mangle]
pub fn decimal_pow() -> i64 {
    let x = Decimal::from(2);
    let exp = 20;
    let z = x.safe_powi(exp).unwrap();
    z.is_positive().into()
}

#[no_mangle]
pub fn decimal_add_call_native() -> i64 {
    let x = Decimal::ONE;
    let mut x_vec = x.to_vec();
    let x_ptr = x_vec.as_mut_ptr();

    let y = Decimal::ONE;
    let mut y_vec = y.to_vec();
    let y_ptr = y_vec.as_mut_ptr();

    let mut z_vec = Vec::<u8>::with_capacity(Decimal::BITS / 8);
    let z_ptr = z_vec.as_mut_ptr();

    unsafe {
        decimal_add_native(x_ptr, y_ptr, z_ptr);
        z_vec.set_len(Decimal::BITS / 8);
    };

    let z = Decimal::try_from(&z_vec[..]).unwrap();
    z.is_positive().into()
}

#[no_mangle]
pub fn decimal_mul_call_native() -> i64 {
    let x = Decimal::ONE;
    let mut x_vec = x.to_vec();
    let x_ptr = x_vec.as_mut_ptr();

    let y = Decimal::ONE;
    let mut y_vec = y.to_vec();
    let y_ptr = y_vec.as_mut_ptr();

    let mut z_vec = Vec::<u8>::with_capacity(Decimal::BITS / 8);
    let z_ptr = z_vec.as_mut_ptr();

    unsafe {
        decimal_mul_native(x_ptr, y_ptr, z_ptr);
        z_vec.set_len(Decimal::BITS / 8);
    };

    let z = Decimal::try_from(&z_vec[..]).unwrap();
    z.is_positive().into()
}

#[no_mangle]
pub fn decimal_pow_call_native() -> i64 {
    let x = Decimal::from(2);
    let mut x_vec = x.to_vec();
    let x_ptr = x_vec.as_mut_ptr();

    let y = 20u32;

    let mut z_vec = Vec::<u8>::with_capacity(Decimal::BITS / 8);
    let z_ptr = z_vec.as_mut_ptr();

    unsafe {
        decimal_pow_native(x_ptr, y as *mut u8, z_ptr);
        z_vec.set_len(Decimal::BITS / 8);
    };

    let z = Decimal::try_from(&z_vec[..]).unwrap();
    z.is_positive().into()
}
