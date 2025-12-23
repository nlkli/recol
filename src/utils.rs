
#[inline(always)]
pub fn as_array_ref<const N: usize>(s: &[u8]) -> &[u8; N] {
    assert_eq!(s.len(), N);
    unsafe { &*(s.as_ptr() as *const [u8; N]) }
}
