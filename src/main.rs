use nom::number::complete::double;
use nom::IResult;

fn main() {
    println!("nom double parse: {:?}", nom_double(". "));
}

fn nom_double(input: &str) -> IResult<&str, f64> {
    double(input)
}
