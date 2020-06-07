use std::io::Error;
use std::process::Command as OsCommand;

/* The command and its arguments */
#[derive(Debug)]
pub struct Command {
	cmd: String,
	args: Vec<String>,
}

impl Command {
	/**
	 * Create a new Command object.
	 *
	 * @param  cmd
	 * @param  args
	 * @return Command
	 */
	pub fn new(cmd: String, args: Vec<String>) -> Self {
		Self { cmd, args }
	}

	/**
	 * Execute the command and wait for it to exit.
	 *
	 * @return Result
	 */
	pub fn execute(&self) -> Result<(), Error> {
		match OsCommand::new(&self.cmd)
			.args(self.args.iter().map(AsRef::as_ref).collect::<Vec<&str>>())
			.spawn()
		{
			Ok(mut child) => {
				child.wait()?;
				Ok(())
			}
			Err(e) => Err(e),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::args::Args;
	use crate::settings::AppSettings;
	use std::time::{Duration, Instant};
	#[test]
	fn test_cmd_mod() -> Result<(), Error> {
		assert_eq!("test", AppSettings::new(&Args::parse()).get_command().cmd);
		let sleep_time = Duration::from_millis(10);
		let now = Instant::now();
		Command::new(String::from("sleep"), vec![String::from("0.01")]).execute()?;
		assert!(now.elapsed() >= sleep_time);
		Ok(())
	}
}
