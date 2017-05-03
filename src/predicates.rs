use std::fmt::{Display, Formatter};
use std::fmt::Error;

pub trait Predicate<T>: Display
    where T: Display
{
    fn name(&self) -> &str;
    fn len(&self) -> usize;
    fn ids(&self) -> &Vec<T>;
    fn is_empty(&self) -> bool;
}

#[derive(Debug)]
pub struct Pred<'a, T> where T: Display {
    name: &'a str,
    ids: Vec<T>,
}

impl <'a, T> Pred<'a, T> where T: Display {
    pub fn new(name: &'a str, ids: Vec<T>) -> Pred<'a, T> {
        Pred{ name: name, ids: ids }
    }
}

#[derive(Debug, PartialEq)]
pub enum ID<'a> {
    Literal(&'a str),
    Variable(&'a str),
}

#[derive(Debug)]
pub enum Stmt<'a> {
    Fact(Pred<'a, ID<'a>>),
    Rule(Pred<'a, ID<'a>>, Vec<Pred<'a, ID<'a>>>),
    Query(Pred<'a, ID<'a>>),
}

impl<'a> Display for Stmt<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Stmt::Fact(ref pred) => write!(f, "{}.", pred),
            Stmt::Rule(ref head, ref tail) => {
                write!(f, "{} :- ", head).unwrap();
                tail.iter().fold(true, |first, elem| {
                    if !first {
                        write!(f, ", ").unwrap();
                    }
                    write!(f, "{}", elem).unwrap();
                    false
                });
                write!(f, ".")
            }
            Stmt::Query(ref pred) => write!(f, "{}?", pred),
        }
    }
}

impl<'a, T> Display for Pred<'a, T>
    where T: Display
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}(", self.name).unwrap();
        for (i, ch) in self.ids.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", ch).unwrap();
            } else {
                write!(f, ",{}", ch).unwrap();
            }
        }
        write!(f, ")")
    }
}

impl<'a> Display for ID<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            ID::Literal(x) => write!(f, "'{}'", x),
            ID::Variable(x) => write!(f, "{}", x),
        }
    }
}

impl<'a, T> Predicate<T> for Pred<'a, T>
    where T: Display
{
    fn len(&self) -> usize {
        self.ids.len()
    }

    fn name(&self) -> &str {
        self.name
    }

    fn ids(&self) -> &Vec<T> {
        &self.ids
    }

    fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::{Stmt, ID, Pred};

    #[test]
    fn queries() {
        let test = Stmt::Query(Pred::new(
                                   "foo",
                                   vec![ID::Literal("bar"), ID::Literal("baz")],
                               ));
        assert_eq!(format!("{}", test), "foo('bar','baz')?");

        let single = Stmt::Query(Pred::new(
                                   "foo",
                                   vec![ID::Literal("bar")],
                               ));
        assert_eq!(format!("{}", single), "foo('bar')?");
    }

    #[test]
    fn facts() {
        let test = Stmt::Fact(Pred::new(
                                   "foo",
                                   vec![ID::Literal("bar"), ID::Literal("baz")],
                               ));
        assert_eq!(format!("{}", test), "foo('bar','baz').");

        let single = Stmt::Fact(Pred::new(
                                   "foo",
                                   vec![ID::Literal("bar")],
                               ));
        assert_eq!(format!("{}", single), "foo('bar').");
    }
}
