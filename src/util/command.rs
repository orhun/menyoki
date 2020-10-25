use std::io::Error;
use std::process::Command as OsCommand;

/* The command and its arguments */
#[derive(Debug)]
pub struct Command<'a> {
	cmd: &'a str,
	args: Vec<&'a str>,
}

/* Implementation for parsing Command from a string */
impl<'a> From<&'a str> for Command<'a> {
	fn from(s: &'a str) -> Command<'a> {
		if !s.contains(' ') {
			Command::new(s, Vec::new())
		} else {
			Command::new("sh", vec!["-c", s])
		}
	}
}

impl<'a> Command<'a> {
	/**
	 * Create a new Command object.
	 *
	 * @param  cmd
	 * @param  args
	 * @return Command
	 */
	pub fn new(cmd: &'a str, args: Vec<&'a str>) -> Self {
		Self { cmd, args }
	}

	/**
	 * Execute the command and wait for it to exit.
	 *
	 * @return Result
	 */
	pub fn execute(&self) -> Result<(), Error> {
		info!("Running the command...");
		match OsCommand::new(&self.cmd).args(&self.args).spawn() {
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
	use std::time::{Duration, Instant};
	#[test]
	fn test_cmd() -> Result<(), Error> {
		let sleep_time = Duration::from_millis(10);
		let now = Instant::now();
		Command::new("sleep", vec!["0.01"]).execute()?;
		assert!(now.elapsed() >= sleep_time);
		Ok(())
	}
}
