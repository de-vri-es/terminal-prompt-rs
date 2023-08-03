fn main() -> std::io::Result<()> {
	let mut terminal = terminal_echo::TerminalPrompter::open()?;
	let username = terminal.prompt("Username: ")?;
	let password = terminal.prompt_sensitive("Password: ")?;
	println!("Username: {username}");
	println!("Password: {password}");
	Ok(())
}
