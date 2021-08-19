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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parser() {
        let test_string = "# Heading\n- List1";
        let parser = Parser::new(test_string);
        let events = parser.collect::<Vec<Event>>();

        assert_eq!(events[0], Event::Heading("Heading"));
        assert_eq!(events[1], Event::List("List1"));
    }
}
