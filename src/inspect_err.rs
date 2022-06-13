pub trait InspectErr<E> {
    fn my_inspect_err<F>(self, f: F) -> Self
    where
        F: Fn(&E);
}

impl<T, E> InspectErr<E> for Result<T, E> {
    fn my_inspect_err<F>(self, f: F) -> Self
    where
        F: Fn(&E),
    {
        self.map_err(|e| {f(&e); e})
    }
}
