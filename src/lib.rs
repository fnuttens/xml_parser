#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str> {
    move |input| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_parser() {
        let parser_joe = match_literal("Hello Joe!");
        assert_eq!(Ok(("", ())), parser_joe("Hello Joe!"));
        assert_eq!(Ok((" Hello Robert!", ())), parser_joe("Hello Joe! Hello Robert!"));
        assert_eq!(Err("Hello Mike!"), parser_joe("Hello Mike!"));
    }
}
