pub trait InspectErr<E> {
    fn inspect_err<F>(self, f: F) -> Self
    where
        F: FnOnce(&E);
}

impl<T, E> InspectErr<E> for Result<T, E> {
    fn inspect_err<F>(self, f: F) -> Self
    where
        F: FnOnce(&E),
    {
        self.map_err(|e| {f(&e); e})
    }
}
