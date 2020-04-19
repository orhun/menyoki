use clap::App;

/**
 * Parse command line arguments.
 *
 * @return ArgMatches
 */
pub fn parse_args() -> clap::ArgMatches<'static> {
	App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.get_matches()
}
