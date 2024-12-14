import d2
import gleeunit
import gleeunit/should

pub fn main() {
  gleeunit.main()
}

pub fn is_safe_test() {
  d2.is_safe([1, 2, 3]) |> should.be_true
  d2.is_safe([3, 2, 1]) |> should.be_true
  d2.is_safe([1, 2, 4]) |> should.be_true
  d2.is_safe([4, 2, 1]) |> should.be_true

  d2.is_safe([1, 2, 6]) |> should.be_false
  d2.is_safe([6, 2, 1]) |> should.be_false
  d2.is_safe([1, 2, 2]) |> should.be_false
  d2.is_safe([2, 2, 1]) |> should.be_false
}
