use std::io::{BufReader, BufRead, Read, Write};

mod sys;

pub struct Terminal {
	terminal: BufReader<sys::Terminal>,
	initial_mode: sys::TerminalMode,
	current_mode: sys::TerminalMode,
}

impl Terminal {
	pub fn open() -> std::io::Result<Self> {
		let terminal = sys::Terminal::open()?;
		let initial_mode = terminal.get_terminal_mode()?;

		let mut current_mode = initial_mode;
		current_mode.enable_line_editing();
		terminal.set_terminal_mode(&current_mode)?;

		Ok(Self {
			terminal: BufReader::new(terminal),
			initial_mode,
			current_mode: initial_mode,
		})
	}

	pub fn is_echo_enabled(&self) -> bool {
		self.current_mode.is_echo_enabled()
	}

	pub fn disable_echo(&mut self) -> std::io::Result<()> {
		self.current_mode.disable_echo();
		self.terminal.get_ref().set_terminal_mode(&self.current_mode)
	}

	pub fn enable_echo(&mut self) -> std::io::Result<()> {
		self.current_mode.enable_echo();
		self.terminal.get_ref().set_terminal_mode(&self.current_mode)
	}

	pub fn read_line(&mut self) -> std::io::Result<String> {
		let mut buffer = String::new();
		self.terminal.read_line(&mut buffer)?;
		if !self.current_mode.is_echo_enabled() {
			writeln!(self).ok();
		}
		if buffer.ends_with('\n') {
			buffer.pop();
		}
		Ok(buffer)
	}

	pub fn prompt(&mut self, prompt: impl std::fmt::Display) -> std::io::Result<String> {
		write!(self, "{prompt}")?;
		self.read_line()
	}

	pub fn prompt_sensitive(&mut self, prompt: impl std::fmt::Display) -> std::io::Result<String> {
		let echo = self.is_echo_enabled();
		self.disable_echo()?;
		write!(self, "{prompt}")?;
		let line = self.read_line();
		if echo {
			self.enable_echo().ok();
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
