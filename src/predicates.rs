use std::fmt::{Display, Formatter};
use std::fmt::Error;

pub trait Predicate<T>: Display
    where T: Display
{
    fn name(&self) -> &str;
    fn len(&self) -> usize;
    fn ids(&self) -> &Vec<T>;
}

pub struct Pred<'a, T> {
    name: &'a str,
    ids: Vec<T>,
}

pub enum ID<'a> {
    Literal(&'a str),
    Variable(&'a str),
}

pub enum Stmt<'a> {
    Fact(Pred<'a, &'a str>),
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
            ID::Literal(ref x) => write!(f, "'{}'", x),
            ID::Variable(ref x) => write!(f, "{}", x),
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
}


#[cfg(test)]
mod tests {
    use super::{Stmt, ID, Pred};

    #[test]
    fn it_works() {
        let test = Stmt::Query(Pred {
                                   name: "foo",
                                   ids: vec![ID::Literal("bar"), ID::Literal("baz")],
                               });
        let string = format!("{}", test);
        assert_eq!(string, "foo('bar','baz')?");
    }
}
