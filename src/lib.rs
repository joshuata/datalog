use std::ops::Index;
use std::slice::Iter;

// ID ( Variable | Constant )
// Predicate (Name, [ID])
// Rule := Predicate :- [Predicate]

enum ID {
	Constant(String),
	Variable(String),
}

enum RuleID {
	Constant(String),
	Variable(String),
	FreeVariable(String),
}

struct Predicate<T> {
	name: String,
	ids: Vec<T>,
}

impl <T> Predicate<T> {
	fn len(&self) -> usize {
		self.ids.len()
	}

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
