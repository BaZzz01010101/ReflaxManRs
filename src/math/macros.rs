macro_rules! impl_binop {
  ($trait:ident::$method:ident for $type:ty, $out:ty) =>{
    impl<T> $trait<T> for $type where T: AsRef<$type> {
      type Output = $out;
       #[inline]
      fn $method(self, other: T) -> $out {
        self.as_ref().$method(other)
      }
    }

    impl<T> $trait<T> for &mut $type where T: AsRef<$type> {
      type Output = $out;
       #[inline]
      fn $method(self, other: T) -> $out {
        self.as_ref().$method(other)
      }
    }
  }
}

macro_rules! impl_binop_assign {
  ($trait:ident::$method:ident for $type:ty) =>{

    impl<T> $trait<T> for $type where T: AsRef<$type> {
       #[inline]
      fn $method(&mut self, other: T) {
        (&mut &mut *self).$method(other)
      }
    }
  }
}
