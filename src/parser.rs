use predicates::ID;
use std::str::from_utf8;

named!(escaped_string,
    escaped!( is_not!("'\\"), '\\', one_of!("\"'\\btnfr") )
);

named!(literal<ID>,
    delimited!(
        tag!("'"),
        map!(map_res!(escaped!( is_not!("'\\"), '\\', one_of!("'\\") ), from_utf8), ID::Literal),
        tag!("'")
    ) 
);

#[cfg(test)]
mod tests {
    use super::*;
    use predicates::ID;
    use std::result::Result::Ok;

    #[test]
    fn test_literal() {
        assert_eq!(literal(b"'asdf'").to_result(), Ok(ID::Literal("asdf")));
        assert_eq!(literal(b"'asd\\'f'").to_result(), Ok(ID::Literal("asd\\'f")));
    }
}