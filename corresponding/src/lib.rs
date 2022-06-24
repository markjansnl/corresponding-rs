pub use corresponding_macros::derive_corresponding;

pub trait MoveCorresponding<R> {
    fn move_corresponding(&mut self, rhs: R);
}
