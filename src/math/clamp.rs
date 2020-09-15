pub fn clamp<T>(value: T, min: T, max: T) -> T
  where T: PartialOrd
{
  let mut result = value;

  if result < min {
    result = min;
  } else if result > max {
    result = max;
  }

  result
}
