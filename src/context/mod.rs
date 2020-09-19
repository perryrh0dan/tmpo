use clap::ArgMatches;

pub struct Context {
  pub yes: bool,
  pub no_script: bool,

}

impl Context {
  pub fn new(args: &ArgMatches) -> Context {
    let mut ctx = Context {
      yes: false,
      no_script: false
    };

    ctx.set_yes(args.is_present("yes"));
    ctx.set_no_script(args.is_present("no-script"));

    return ctx;
  }

  fn set_yes(&mut self, yes: bool) {
    self.yes = yes;
  }

  fn set_no_script(&mut self, ns: bool) {
    self.no_script = ns;
  }
}
