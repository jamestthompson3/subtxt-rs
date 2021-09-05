pub struct Parser<'a> {
    input: std::str::Lines<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            input: text.lines(),
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Event<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.input.next() {
            Some(line) => {
                if line.is_empty() {
                    return Some(Event::Empty);
                }
                let mut chars = line.chars();
                match (chars.next().unwrap(), chars.next().unwrap()) {
                    ('#', ' ') => Some(Event::Heading(line.split_at(2).1)),
                    ('-', ' ') => Some(Event::List(line.split_at(2).1)),
                    ('&', ' ') => Some(Event::Link(line.split_at(2).1)),
                    ('>', ' ') => Some(Event::Quote(line.split_at(2).1)),
                    _ => Some(Event::Text(line)),
                }
            }
            None => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Event<'a> {
    Heading(&'a str),
    Text(&'a str),
    List(&'a str),
    Quote(&'a str),
    Link(&'a str),
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_normal_input() {
        let test_string = "# Heading\n- List1";
        let parser = Parser::new(test_string);
        let events = parser.collect::<Vec<Event>>();

        assert_eq!(events[0], Event::Heading("Heading"));
        assert_eq!(events[1], Event::List("List1"));
    }

    #[test]
    fn parse_malformed_input() {
        let test_string = "# ";
        let parser = Parser::new(test_string);
        let events = parser.collect::<Vec<Event>>();

        assert_eq!(events[0], Event::Heading(""));
    }

    #[test]
    fn parse_malformed_and_normal_input() {
        let test_string = "# \n- List1";
        let parser = Parser::new(test_string);
        let events = parser.collect::<Vec<Event>>();

        assert_eq!(events[0], Event::Heading(""));
        assert_eq!(events[1], Event::List("List1"));
    }

    #[test]
    fn parse_typo_input() {
        let test_string = "-List1";
        let parser = Parser::new(test_string);
        let events = parser.collect::<Vec<Event>>();

        assert_eq!(events[0], Event::Text("-List1"));
    }

    #[test]
    fn parse_empty_lines() {
        let test_string = "asdf\n\nasdf";
        let parser = Parser::new(test_string);
        let events = parser.collect::<Vec<Event>>();

        assert_eq!(events[1], Event::Empty);
    }
}
