use rand::rngs::StdRng;
use std::cell::RefCell;

thread_local! {
    static RNG: RefCell<Option<StdRng>> = const { RefCell::new(None) }
}

/// Sets the RNG
pub fn set_rng(rng: StdRng) {
    RNG.with(|cell| cell.borrow_mut().replace(rng));
}

/// Get the RNG
///
/// # Panics
///
/// When no rng is there.
///
pub fn get_rng() -> &'static mut StdRng {
    RNG.with(|cell| {
        let mut cell = cell.borrow_mut();
        let rng = cell.as_mut().expect("Where is my RNG ?");
        let rng: *mut StdRng = rng;

        let rng: &'static mut StdRng = unsafe { &mut *rng };
        rng
    })
}
