use std::io;

#[inline(always)]
pub fn as_array_ref<T, const N: usize>(s: &[T]) -> &[T; N] {
    assert_eq!(s.len(), N);
    unsafe { &*(s.as_ptr() as *const [T; N]) }
}


pub fn io_other_error<E>(err: E) -> io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, err)
}

pub fn missing_field(path: &'static str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("required field `{}` is missing", path),
    )
}

#[macro_export]
macro_rules! require_field {
    ($root:expr, $path:literal, $field:ident) => {
        $root.$field
            .as_ref()
            .ok_or_else(|| missing_field($path))
    };
}
