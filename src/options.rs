use getopts::Options;
use ansi_term::Colour::{Yellow};

/// parse possible commandline options
pub fn parse_commandline_options(args: &Vec<String>) {
    debug!("parsing options ...");

    let mut opts = Options::new();
    opts.optopt("i", "interface", "which interface to listen", "INTERFACE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
    	debug!("found commandline-option: `{}`", "h");
        print_usage(&args[0].clone(), opts);
        return;
    }

    let output = matches.opt_str("i");
    let input = if !matches.free.is_empty() {
		warn!("found commandline-option: `i` without a value");
        matches.free[0].clone()
    } else {
    	// Take a reference to the contained string
		match output {
		    Some(ref m) => debug!("found commandline-option: `{}`: {}", "i", *m),
		    None => (),
		}
        // print_usage(&args[0].clone(), opts);
        // Yellow.paint("i")
        return;
    };

}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}