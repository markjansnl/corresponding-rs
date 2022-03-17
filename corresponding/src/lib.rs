pub use corresponding_macros::derive_corresponding;

//------- Traits

pub trait MoveCorresponding<R> {
    fn move_corresponding(&mut self, rhs: R);
}

pub trait CloneCorresponding<R> {
    fn clone_corresponding(&mut self, rhs: &R);
}

pub trait CopyCorresponding<R> {
    fn copy_corresponding(&mut self, rhs: &R);
}

//------- Generic implementations

impl<R> MoveCorresponding<R> for R {
    #[inline]     
    fn move_corresponding(&mut self, rhs: R) {
        *self = rhs;
    }
}
impl<L: MoveCorresponding<R>, R: Clone> CloneCorresponding<R> for L {
    #[inline]     
    fn clone_corresponding(&mut self, rhs: &R) {
        self.move_corresponding(rhs.clone());
    }
}
impl<L: MoveCorresponding<R>, R: Copy> CopyCorresponding<R> for L {
    #[inline]     
    fn copy_corresponding(&mut self, rhs: &R) {
        self.move_corresponding(*rhs);
    }
}