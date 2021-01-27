#![no_std]

use core::mem::MaybeUninit;

/// Allocates `[u8;size]` memory on stack and invokes `closure` with this slice as argument.
///
///
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
#[allow(nonstandard_style)]
pub fn with_alloca<R, F>(size: usize, f: F) -> R
where
    F: FnOnce(&mut [MaybeUninit<u8>]) -> R,
{
    unsafe {
        use ::core::ffi::c_void;
        type cb_t = unsafe extern "C" fn(size: usize, ptr: *mut u8, data: *mut c_void);
        extern "C" {
            fn c_with_alloca(size: usize, cb: cb_t, data: *mut c_void);
        }
        let mut f = Some(f);
        let mut ret = None::<R>;
        // &mut (impl FnMut(*mut u8))
        let ref mut f = |ptr: *mut u8| {
            let slice = core::mem::transmute::<&mut [u8], &mut [MaybeUninit<u8>]>(
                ::core::slice::from_raw_parts_mut(ptr, size),
            );

            ret = Some(f.take().unwrap()(slice));
        };
        fn with_F_of_val<F>(_: &mut F) -> cb_t
        where
            F: FnMut(*mut u8),
        {
            unsafe extern "C" fn trampoline<F: FnMut(*mut u8)>(
                _size: usize,
                ptr: *mut u8,
                data: *mut c_void,
            ) {
                (&mut *data.cast::<F>())(ptr);
            }

            trampoline::<F>
        }

        c_with_alloca(size, with_F_of_val(f), <*mut _>::cast::<c_void>(f));

        ret.unwrap()
    }
}

#[cfg(test)]
mod tests;
