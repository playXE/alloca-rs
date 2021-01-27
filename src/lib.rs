#![no_std]
extern crate alloc;
use alloc::boxed::Box;

extern "C" {
    fn c_with_alloca(
        _: usize,
        _: unsafe extern "C" fn(_: usize, _: *mut u8, _: *mut u8) -> *mut u8,
        _: *mut u8,
    ) -> *mut u8;
}
unsafe extern "C" fn trampoline(size: usize, ptr: *mut u8, data: *mut u8) -> *mut u8 {
    let data = &mut *data.cast::<Data>();
    (data.closure.take().unwrap())(core::slice::from_raw_parts_mut(ptr, size))
}

struct Data {
    closure: Option<Box<dyn FnOnce(&mut [u8]) -> *mut u8>>,
}

impl Data {
    pub fn new<R>(clos: impl FnOnce(&mut [u8]) -> R + 'static) -> Self {
        let boxed = move |memory: &mut [u8]| {
            let result = Box::into_raw(Box::new(clos(memory))) as *mut u8;
            result
        };
        Self {
            closure: Some(Box::new(boxed)),
        }
    }
}

/// Allocates `[u8;size]` memory on stack and invokes `closure` with this slice as argument.
///
///
/// NOTE: Memory slice is not guaranteed to be zeroed. Probably on every OS it will have uninitialized values.
/// # Safety
/// This function is safe because `c_with_alloca` (which is internally used) will always returns non-null
/// pointer.
///
/// # Potential segfaults or UB
///
/// When using this function in wrong way your program might get UB or segfault "for free":
/// - Using memory allocated by `with_alloca` outside of it e.g closure is already returned but you somehow
/// managed to store pointer to memory and use it.
/// - Allocating more memory than thread stack size.
///     
///
///     This will trigger segfault on stack overflow.
///
///
///
pub fn with_alloca<R>(size: usize, closure: impl FnOnce(&mut [u8]) -> R + 'static) -> R {
    let mut data = Data::new(closure);

    unsafe {
        let result = c_with_alloca(size, trampoline, &mut data as *mut Data as *mut u8);
        *Box::from_raw(result.cast::<R>())
    }
}

#[cfg(test)]
mod tests;
