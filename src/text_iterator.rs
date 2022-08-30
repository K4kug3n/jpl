#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Symbol {
	pub value: char,
	pub col: usize,
	pub line: usize,
}

impl Symbol {
	fn new(value: char, col: usize, line: usize) -> Symbol {
		Symbol{
			value: value,
			col: col,
			line: line
		}
	}
}

pub struct TextIterator<'a> {
	raw_iterator : std::str::Chars<'a>,
	
	current_col: usize,
	current_line: usize
}

impl TextIterator<'_> {
	pub fn new(text: &str) -> TextIterator {
		TextIterator {
			raw_iterator: text.chars(),
			current_col: 0,
			current_line: 0,
		}
	}

	pub fn next(&mut self) -> Option<Symbol> {
		let opt_value = self.raw_iterator.next();

		if let Some(value) = opt_value {
			let col = self.current_col;
			self.current_col += 1;

			let line = self.current_line;
			if value == '\n' {
				self.current_line += 1;
				self.current_col = 0;
			}

			return Some(Symbol::new(value, col, line));
		}
		
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn iterate() {
		let mut text_iterator = TextIterator::new("a b\nc");

		assert_eq!(text_iterator.next(), Some(Symbol::new('a', 0, 0)));
		assert_eq!(text_iterator.next(), Some(Symbol::new(' ', 1, 0)));
		assert_eq!(text_iterator.next(), Some(Symbol::new('b', 2, 0)));
		assert_eq!(text_iterator.next(), Some(Symbol::new('\n', 3, 0)));
		assert_eq!(text_iterator.next(), Some(Symbol::new('c', 0, 1)));
		assert_eq!(text_iterator.next(), None);
	}
}