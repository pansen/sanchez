use getopts::Options;
// use ansi_term::Colour::{Yellow};

/// parse possible commandline options
pub fn parse_commandline_options(args: &Vec<String>) -> Result<i16, String>{
    debug!("parsing options ...");

    let mut opts = Options::new();
    opts.optopt("i", "interface", "which interface to listen", "INTERFACE");
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

    let output = matches.opt_str("i");
    if matches.free.is_empty() {
    	// resolve `core::option::Option` to a string ref
    	// https://doc.rust-lang.org/core/option/index.html#examples
		match output {
		    Some(ref m) => debug!("found commandline-option: `{}`: {}", "i", *m),
		    None => (),
		}
        // print_usage(&args[0].clone(), opts);
        // Yellow.paint("i")
		return Ok(0);
    }

	warn!("no idea: {}", matches.free[0].clone());
    return Err("no idea".to_string());
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}