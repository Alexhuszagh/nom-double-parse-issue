use nom::number::complete::double;
use nom::IResult;
use lexical_043;
use lexical_070;

fn main() {
    let values = [
        ".",
        ". ",
        ".1 "
    ];
    for value in values.iter() {
        println!("Parsing value: '{:?}'", value);
        println!("  nom double parse: {:?}", nom_double(value));
        println!("  lexical_043 double parse: {:?}", lexical_043_double(value));
        println!("  lexical_070 double parse: {:?}", lexical_070_double(value));
    }
}

fn lexical_043_double(input: &str) -> lexical_043::Result<f64> {
    lexical_043::try_atof64_slice(input.as_bytes())
}

fn lexical_070_double(input: &str) -> lexical_070::Result<(f64, usize)> {
    lexical_070::parse_partial(input.as_bytes())
}

fn nom_double(input: &str) -> IResult<&str, f64> {
    double(input)
}
