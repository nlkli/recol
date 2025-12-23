
#[inline(always)]
pub fn as_array_ref<T, const N: usize>(s: &[T]) -> &[T; N] {
    assert_eq!(s.len(), N);
    unsafe { &*(s.as_ptr() as *const [T; N]) }
}
