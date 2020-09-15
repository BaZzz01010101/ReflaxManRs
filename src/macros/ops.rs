macro_rules! impl_binop_internal {
  ($trait:ident::$method:ident for [$left:ty, $right:ty] => $out:ty as [$left_base:ty, $right_base:ty]) => {

    impl $trait<$right> for $left {
      type Output = $out;
       #[inline]
      fn $method(self, other: $right) -> $out {
        <$left_base as $trait::<$right_base>>::$method(&self, &other)
      }
    }

  }
}

macro_rules! impl_binop {
  ($trait:ident::$method:ident for [$left:ty, $right:ty] => $out:ty) => {

    impl_binop_internal!($trait::$method for [$left, $right] => $out as [&$left, &$right]);
    impl_binop_internal!($trait::$method for [$left, &mut $right] => $out as [&$left, &$right]);
    impl_binop_internal!($trait::$method for [$left, &$right] => $out as [&$left, &$right]);

    impl_binop_internal!($trait::$method for [&mut $left, $right] => $out as [&$left, &$right]);
    impl_binop_internal!($trait::$method for [&mut $left, &mut $right] => $out as [&$left, &$right]);
    impl_binop_internal!($trait::$method for [&mut $left, &$right] => $out as [&$left, &$right]);

    impl_binop_internal!($trait::$method for [&$left, $right] => $out as [&$left, &$right]);
    impl_binop_internal!($trait::$method for [&$left, &mut $right] => $out as [&$left, &$right]);
  }
}

macro_rules! impl_op_assign_internal {
  ($trait:ident::$method:ident for [$left:ty, $right:ty] as [$left_base:ty, $right_base:ty]) => {

    impl $trait<$right> for $left {
       #[inline]
      fn $method(&mut self, other: $right) {
        <$left_base as $trait::<$right_base>>::$method(&mut&mut *self, &other)
      }
    }

  }
}

macro_rules! impl_op_assign {
  ($trait:ident::$method:ident for [$left:ty, $right:ty]) => {
    impl_op_assign_internal!($trait::$method for [$left, $right] as [&mut $left, &$right]);
    impl_op_assign_internal!($trait::$method for [$left, &$right] as [&mut $left, &$right]);
    impl_op_assign_internal!($trait::$method for [$left, &mut $right] as [&mut $left, &$right]);
    impl_op_assign_internal!($trait::$method for [&mut $left, $right] as [&mut $left, &$right]);
    impl_op_assign_internal!($trait::$method for [&mut $left, &mut $right] as [&mut $left, &$right]);
  }
}

macro_rules! impl_op_internal {
  ($trait:ident::$method:ident for $type:ty => $out:ty as $base:ty) => {

    impl $trait for $type {
      type Output = $out;
       #[inline]
      fn $method(self) -> $out {
        <$base as $trait>::$method(&self)
      }
    }

  }
}

macro_rules! impl_op {
  ($trait:ident::$method:ident for $type:ty => $out:ty) => {
    impl_op_internal!($trait::$method for $type => $out as &$type);
    impl_op_internal!($trait::$method for &mut $type => $out as &$type);
  }
}

