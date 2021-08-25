use robust_binary_search::*;

#[no_mangle]
pub unsafe extern "C" fn autosearcher_init(len: usize) -> *mut AutoSearcher {
    Box::into_raw(Box::new(AutoSearcher::new(len)))
}

#[no_mangle]
pub unsafe extern "C" fn autosearcher_destroy(s: *mut AutoSearcher) {
    Box::from_raw(s);
}

#[no_mangle]
pub unsafe extern "C" fn autosearcher_report(s: *mut AutoSearcher, index: usize, heads: bool) {
    (&mut *s).report(index, heads);
}

#[no_mangle]
pub unsafe extern "C" fn autosearcher_next_index(s: *const AutoSearcher) -> usize {
    (&*s).next_index()
}

#[no_mangle]
pub unsafe extern "C" fn autosearcher_best_index(s: *const AutoSearcher) -> usize {
    (&*s).best_index()
}

#[no_mangle]
pub unsafe extern "C" fn autosearcher_likelihood(s: *const AutoSearcher, index: usize) -> f64 {
    (&*s).likelihood(index)
}
