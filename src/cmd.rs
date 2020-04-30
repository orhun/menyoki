use clap::ArgMatches;
use std::io::Error;
use std::process::Command as OsCommand;

#[derive(Debug)]
pub struct Command {
	cmd: String,
	args: Vec<String>,
}

impl Command {
	pub fn new(cmd: String, args: Vec<String>) -> Self {
		Self { cmd, args }
	}

	/**
	 * Get a Command object from parsed arguments.
	 *
	 * @param  args
	 * @return Command
	 */
	pub fn get(args: &ArgMatches) -> Self {
		match args.value_of("command") {
			Some(cmd) => {
				let cmd = String::from(cmd);
				if !cmd.contains(' ') {
					Command::new(cmd, Vec::new())
				} else {
					Command::new(String::from("sh"), vec![String::from("-c"), cmd])
				}
			}
			None => panic!("No command specified to run"),
		}
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
	use std::time::{Duration, Instant};
	#[test]
	fn test_exec_cmd() -> Result<(), Error> {
		let sleep_time = Duration::from_millis(10);
		let now = Instant::now();
		exec_cmd("sleep", &["0.01"])?;
		assert!(now.elapsed() >= sleep_time);
		Ok(())
	}
}
