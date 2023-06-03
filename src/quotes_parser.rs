use chumsky::prelude::*;

pub fn parse(s: &str) -> Option<Vec<Quote>> {
    match guessing_parser().parse(s) {
        Ok(result) if result.len() >= 2 => Some(result),
        Ok(_result) => {
            println!("File contained less than 2 quotes. Maybe it isn't a quotes file?");
            None
        }
        Err(err) => {
            println!("{err:?}");
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Quote {
    pub author: Option<String>,
    pub text: String,
}

impl Quote {
    pub fn new(author: &str, text: &str) -> Self {
        Quote {
            author: Some(author.trim().to_owned()),
            text: text.trim().to_owned(),
        }
    }

    pub fn new_anonymous(text: &str) -> Self {
        Quote {
            author: None,
            text: text.trim().to_owned(),
        }
    }
}

mod thunderbird_parser {
    use super::Quote;
    use chumsky::{prelude::*, text::newline};

    fn line_to_quote(line: String) -> Quote {
        let line = line.replace('\n', "").replace("<br>", "\n");
        match line.rsplit_once("- ") {
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
mod mail_quotes_parser {
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

fn guessing_parser() -> impl Parser<char, Vec<Quote>, Error = Simple<char>> {
    mail_quotes_parser::parser().or(thunderbird_parser::parser())
}

#[test]
fn test_parse_thunderbird_quotes() {
    let text = "\
        Some quote. - A. N. Other\
        \n%\n\
        Another quote.- Author Name\
        \n%\n\
        Quote3.-Different Name\
        \n%\n\
        Multiline quote<br>\n\
        is a quote on many lines - Some Author\
        \n%\n\
        Some quote without an author\
    ";
    assert_eq!(
        thunderbird_parser::parser().parse(text),
        Ok(vec![
            Quote::new("A. N. Other", "Some quote."),
            Quote::new("Author Name", "Another quote."),
            Quote::new("Different Name", "Quote3."),
            Quote::new("Some Author", "Multiline quote\nis a quote on many lines"),
            Quote::new_anonymous("Some quote without an author"),
        ])
    );
}

#[test]
fn test_parse_mail_quotes() {
    let text = "\
        Some quote.:A. N. Other:\
        \n\n\
        Another quote. :Author Name:\
        \n\n\
        Quote3.:Different Name:\
        \n\n\
        [This is a multiline\nquote :Some Name:]\
        \n\n\
        [Multiline quote\nWithout an author]\
        \n\n\
        Regular quote without an author\
    ";
    assert_eq!(
        mail_quotes_parser::parser().parse(text),
        Ok(vec![
            Quote::new("A. N. Other", "Some quote."),
            Quote::new("Author Name", "Another quote."),
            Quote::new("Different Name", "Quote3."),
            Quote::new("Some Name", "This is a multiline\nquote"),
            Quote::new_anonymous("Multiline quote\nWithout an author"),
            Quote::new_anonymous("Regular quote without an author"),
        ])
    );
}

#[test]
fn test_guess_file_type_thunderbird() {
    let text = "\
    Some quote. - A. N. Other\
    \n%\n\
    Another quote.- Author Name\
    ";
    assert_eq!(
        parse(text),
        Some(vec![
            Quote::new("A. N. Other", "Some quote."),
            Quote::new("Author Name", "Another quote."),
        ])
    );
}

#[test]
fn test_guess_file_type_mail_quotes() {
    let text = "\
    Some quote.:A. N. Other:\
    \n\n\
    Another quote. :Author Name:\
    ";
    assert_eq!(
        parse(text),
        Some(vec![
            Quote::new("A. N. Other", "Some quote."),
            Quote::new("Author Name", "Another quote."),
        ])
    );
}

#[test]
fn test_guess_file_type_unidentified() {
    let text = "\
    Random text here\
    ";
    assert_eq!(parse(text), None);
}
