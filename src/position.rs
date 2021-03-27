use std::fmt;
/**
 * the doc's position
*/
#[derive(Debug)]
pub struct CodeRegion {
	index: usize,
	line: usize,
	col: usize,
}

impl CodeRegion {
	pub(crate) fn from_context_index(context: &str, index: usize) -> Self {
		let mut region = CodeRegion {
			index,
			line: 1,
			col: 0,
		};
		let mut prev_char = '\0';
		for (cur_index, c) in context.chars().into_iter().enumerate() {
			if cur_index <= index {
				let mut need_move_col = true;
				// \r newline in early macos
				if c == '\r' {
					region.set_new_line();
					need_move_col = false;
				} else if c == '\n' {
					// \n in windows, combine \r\n as newline
					if prev_char == '\r' {
						// do nothing, because did in \r
					} else {
						// set to nextline
						region.set_new_line();
					}
					need_move_col = false;
				}
				// move one col for the code region
				if need_move_col {
					region.move_one();
				}
				prev_char = c;
			}
		}
		region
	}

	// jump to new line
	pub fn set_new_line(&mut self) {
		self.line += 1;
		self.col = 0;
	}
	// move to next col
	pub fn move_one(&mut self) {
		self.col += 1;
		self.index += 1;
	}
}

impl fmt::Display for CodeRegion {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let output = format!("[line:{},col:{},index:{}]", self.line, self.col, self.index);
		f.write_str(output.as_str())
	}
}
