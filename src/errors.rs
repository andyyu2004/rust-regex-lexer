
pub trait Catch {
    type Item;
    fn catch<F>(self, f: F) where F: FnOnce(Self::Item) -> ();
}

// map_left but doesn't return anything
// Can probably be done with if let Err(e) = ...
impl<T, E> Catch for Result<T, E> {
    type Item = E;

    fn catch<F>(self, f: F) where F: FnOnce(Self::Item) -> () {
        match self {
            Err(e) => f(e),
            Ok(_) => {}
        };
    }
}
