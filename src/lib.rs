use std::io::{BufReader, BufRead, Read, Write};

mod sys;

pub struct Terminal {
	terminal: BufReader<sys::Terminal>,
	initial_mode: sys::TerminalMode,
}

impl Terminal {
	pub fn open() -> std::io::Result<Self> {
		let terminal = sys::Terminal::open()?;
		let initial_mode = terminal.get_terminal_mode()?;

		// Enable line editing mode.
		let mut mode = initial_mode;
		mode.enable_line_editing();
		terminal.set_terminal_mode(&mode)?;

		Ok(Self {
			terminal: BufReader::new(terminal),
			initial_mode,
		})
	}

	pub fn is_echo_enabled(&self) -> std::io::Result<bool> {
		let mode = self.terminal.get_ref().get_terminal_mode()?;
		Ok(mode.is_echo_enabled())
	}

	pub fn disable_echo(&self) -> std::io::Result<()> {
		let mut mode = self.terminal.get_ref().get_terminal_mode()?;
		mode.disable_echo();
		self.terminal.get_ref().set_terminal_mode(&mode)?;
		Ok(())
	}

	pub fn enable_echo(&mut self) -> std::io::Result<()> {
		let mut mode = self.terminal.get_ref().get_terminal_mode()?;
		mode.enable_echo();
		self.terminal.get_ref().set_terminal_mode(&mode)?;
		Ok(())
	}

	pub fn read_input_line(&mut self) -> std::io::Result<String> {
		let mut buffer = String::new();
		self.terminal.read_line(&mut buffer)?;

		if self.is_echo_enabled().unwrap_or(false) {
			writeln!(self).ok();
		}
		if buffer.ends_with('\n') {
			buffer.pop();
		}
		Ok(buffer)
	}

	pub fn prompt(&mut self, prompt: impl std::fmt::Display) -> std::io::Result<String> {
		write!(self, "{prompt}")?;
		self.read_input_line()
	}

	pub fn prompt_sensitive(&mut self, prompt: impl std::fmt::Display) -> std::io::Result<String> {
		let old_mode = self.terminal.get_ref().get_terminal_mode()?;
		if old_mode.is_echo_enabled() {
			let mut new_mode = old_mode;
			new_mode.disable_echo();
			self.terminal.get_ref().set_terminal_mode(&new_mode)?;
		}
		write!(self, "{prompt}")?;
		let line = self.read_input_line();
		if old_mode.is_echo_enabled() {
			self.terminal.get_ref().set_terminal_mode(&old_mode).ok();
		}
		line
	}
}

impl Drop for Terminal {
	fn drop(&mut self) {
		self.terminal.get_ref().set_terminal_mode(&self.initial_mode).ok();
	}
}

impl Read for Terminal {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		self.terminal.read(buf)
	}

	fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
		self.terminal.read_vectored(bufs)
	}
}

impl BufRead for Terminal {
	fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
		self.terminal.fill_buf()
	}

	fn consume(&mut self, amt: usize) {
		self.terminal.consume(amt)
	}
}

impl Write for Terminal {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.terminal.get_mut().write(buf)
	}

	fn flush(&mut self) -> std::io::Result<()> {
		self.terminal.get_mut().flush()
	}

	fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
		self.terminal.get_mut().write_vectored(bufs)
	}
}
