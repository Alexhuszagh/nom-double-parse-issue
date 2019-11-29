use nom::number::complete::{double, recognize_float};
use nom::{IResult, Slice, Offset, InputIter, InputLength, AsChar, InputTakeAtPosition, ParseTo};
use nom::error::{ParseError, ErrorKind};
use std::ops::{RangeFrom, RangeTo};

fn main() {
    println!("nom double parse: {:?}", nom_double(". "));
    println!("my double parse: {:?}", my_double(". "));
}

fn nom_double(input: &str) -> IResult<&str, f64> {
    double(input)
}

fn my_double(input: &str) -> IResult<&str, f64> {
    double2(input)
}

pub fn double2<T, E:ParseError<T>>(input: T) -> IResult<T, f64, E>
    where
        T: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
        T: Clone + Offset,
        T: InputIter + InputLength + nom::ParseTo<f64>,
        <T as InputIter>::Item: AsChar,
        T: InputTakeAtPosition,
        <T as InputTakeAtPosition>::Item: AsChar
{
    match recognize_float(input) {
        Err(e) => Err(e),
        Ok((i, s)) => match s.parse_to() {
            Some(n) => Ok((i, n)),
            None =>  Err(nom::Err::Error(E::from_error_kind(i, ErrorKind::Float)))
        }
    }
}
