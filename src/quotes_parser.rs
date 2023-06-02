#[derive(Debug, Eq, PartialEq)]
pub struct Quote {
    author: Option<String>,
    text: String,
}

impl Quote {
    fn new(author: &str, text: &str) -> Self {
        Quote {
            author: Some(author.trim().to_owned()),
            text: text.trim().to_owned(),
        }
    }

    fn new_anonymous(text: &str) -> Self {
        Quote {
            author: None,
            text: text.trim().to_owned(),
        }
    }
}

mod tb_parser {
    use super::Quote;
    use chumsky::{prelude::*, text::newline};

    fn line_to_quote(line: String) -> Quote {
        let line = line.replace('\n', "").replace("<br>", "\n");
        match line.rsplit_once('-') {
            Some((text, author)) => Quote::new(author, text),
            None => Quote::new_anonymous(&line),
        }
    }

    pub fn parser() -> impl Parser<char, Vec<Quote>, Error = Simple<char>> {
        let quote_sep = newline().then(just('%')).then(newline());
        let line = quote_sep
            .clone()
            .not()
            .repeated()
            .at_least(1)
            .collect::<String>();
        line.map(|l| line_to_quote(l))
            .separated_by(quote_sep)
            .then_ignore(end())
            .collect()
    }
}
mod quotes_parser {
    use super::Quote;
    use chumsky::{prelude::*, text::newline};

    fn line_to_quote(line: String) -> Quote {
        let parts: Vec<_> = line.rsplitn(3, ':').collect();
        if parts.len() == 3 {
            let author = parts[1];
            let text = parts[2];
            Quote::new(author, text)
        } else {
            Quote::new_anonymous(line.as_str())
        }
    }

    pub fn parser() -> impl Parser<char, Vec<Quote>, Error = Simple<char>> {
        let simple_line = newline().not().repeated().at_least(1).collect::<String>();
        let multi_line = just(']')
            .not()
            .repeated()
            .delimited_by(just('['), just(']'))
            .collect::<String>();
        multi_line
            .or(simple_line)
            .map(|line| line_to_quote(line))
            .separated_by(newline().repeated().exactly(2))
            .allow_trailing()
            .then_ignore(end())
    }
}

#[test]
fn header_test() {
    use chumsky::prelude::*;
    use std::str::from_utf8;

    let quotes = include_bytes!("../Quotes.txt");
    let tbquotes = include_bytes!("../TBQuotes.txt");
    let tbq = from_utf8(tbquotes).unwrap();
    println!("{:#?}", tb_parser::parser().parse(tbq));

    let q = from_utf8(quotes).unwrap();
    println!("{:#?}", quotes_parser::parser().parse(q));
}
