use getopts::Options;
// use ansi_term::Colour::{Yellow};

/// Data holder for possible flags or actions which are parsed in `parse_commandline_options`
pub struct Command {
    pub list: bool,
    pub iface: String,
    pub port: i32,
}


/// parse possible commandline options
/// see: https://doc.rust-lang.org/getopts/getopts/index.html
pub fn parse_commandline_options(args: &Vec<String>) -> Result<Command, String>{
    debug!("parsing options ...");

    let mut opts = Options::new();
    let mut command: Command = Command{list: false, iface: "".to_string(), port: -1};

    opts.optopt("i", "interface", "which interface to listen", "INTERFACE");
    opts.optopt("p", "port", "which port to listen", "PORT");
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

    if matches.opt_present("i") {
    	// resolve `core::option::Option` to a string ref
    	// see: https://doc.rust-lang.org/core/option/index.html#examples
		match matches.opt_str("i") {
		    Some(ref m) => {
		        debug!("found commandline-option: `{}`: {}", "i", *m);
		        // create a copy of that string ref: http://stackoverflow.com/a/26916753
		    	command.iface = m.trim().chars().collect();
		    },
		    None => (),
		}
    }

    if matches.opt_present("p") {
    	// resolve `core::option::Option` to a string ref
    	// see: https://doc.rust-lang.org/core/option/index.html#examples
		match matches.opt_str("p") {
		    Some(ref m) => {
		    	debug!("found commandline-option: `{}`: {}", "p", m.trim());
		    	let port = m.trim().parse::<i32>().ok().expect("Failed to cast port");
		    	command.port = port;
		    },
		    None => (),
		}
    }

	return Ok(command);
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
	use logging;

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

	#[test]
	fn test_parse_commandline_options_interface() {
		let args = vec!("main".to_string(), "-ieth0".to_string());
		let command:Command;

	    match parse_commandline_options(&args) {
	        Ok(c) => {command = c;}
	        Err(f) => { panic!(f) }
	    };
		assert_eq!(false, command.list);
		assert_eq!("eth0", command.iface);
		assert_eq!(-1, command.port);
	}

	#[test]
	fn test_parse_commandline_options_port() {
		let args = vec!("main".to_string(), "-p8080".to_string());
		let command:Command;

	    match parse_commandline_options(&args) {
	        Ok(c) => {command = c;}
	        Err(f) => { panic!(f) }
	    };
		assert_eq!(false, command.list);
		assert_eq!("", command.iface);
		assert_eq!(8080, command.port);
	}

	#[test]
	fn test_parse_commandline_options_interface_and_port() {
		logging::setup_logging();
		let args = vec!("main".to_string(), "-i eth0".to_string(), "-p 8080".to_string());
		let command:Command;

	    match parse_commandline_options(&args) {
	        Ok(c) => {command = c;}
	        Err(f) => { panic!(f) }
	    };
		assert_eq!(false, command.list);
		assert_eq!("eth0", command.iface);
		assert_eq!(8080, command.port);
	}
}