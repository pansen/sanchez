use getopts::Options;
// use ansi_term::Colour::{Yellow};

/// Data holder for possible flags or actions which are parsed in `parse_commandline_options`
pub struct Command {
    pub list: bool,
}


/// parse possible commandline options
/// see: https://doc.rust-lang.org/getopts/getopts/index.html
pub fn parse_commandline_options(args: &Vec<String>) -> Result<Command, String>{
    debug!("parsing options ...");

    let mut opts = Options::new();
    let mut command: Command = Command{list: false};

    opts.optopt("i", "interface", "which interface to listen", "INTERFACE");
    opts.optflag("l", "list", "list possible devices to work with");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { return Err(f.to_string()) }
    };

    if matches.opt_present("h") {
    	debug!("found commandline-option: `{}`", "h");
        print_usage(&args[0].clone(), opts);
        return Err("help called".to_string());
    }

    if matches.opt_present("l") {
    	debug!("found commandline-option: `{}`", "l");
    	command.list = true;
        return Ok(command);
    }

    let output = matches.opt_str("i");
    if matches.free.is_empty() {
    	// resolve `core::option::Option` to a string ref
    	// see: https://doc.rust-lang.org/core/option/index.html#examples
		match output {
		    Some(ref m) => debug!("found commandline-option: `{}`: {}", "i", *m),
		    None => (),
		}
        // print_usage(&args[0].clone(), opts);
        // Yellow.paint("i")
		return Ok(command);
    }

	warn!("no idea: {}", matches.free[0].clone());
    return Err("no idea".to_string());
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

/// tests. yeah.
/// see: https://github.com/rust-lang-nursery/getopts/blob/master/src/lib.rs#L1035
#[cfg(test)] #[macro_use]
mod tests {
	use super::{parse_commandline_options, Command};

	#[test]
	fn test_parse_commandline_options_no_options() {
		let args = vec!("main".to_string());
		let command:Command;

	    match parse_commandline_options(&args) {
	        Ok(c) => {command = c;}
	        Err(f) => { panic!(f) }
	    };
		assert_eq!(false, command.list);
	}

	#[test]
	fn test_parse_commandline_options_list() {
		let args = vec!("main".to_string(), "-l".to_string());
		let command:Command;

	    match parse_commandline_options(&args) {
	        Ok(c) => {command = c;}
	        Err(f) => { panic!(f) }
	    };
		assert_eq!(true, command.list);
	}
}