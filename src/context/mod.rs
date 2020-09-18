use clap::ArgMatches;

pub struct Context {
  pub no_script: bool,
}

impl Context {
  pub fn new(args: &ArgMatches) -> Context {
    let mut ctx = Context { no_script: false };

    ctx.set_no_script(args.is_present("no-script"));

    return ctx;
  }

  fn set_no_script(&mut self, ns: bool) {
    self.no_script = ns;
  }
}
