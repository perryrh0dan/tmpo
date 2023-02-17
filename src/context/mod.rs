use clap::ArgMatches;

pub struct Context {
  pub yes: bool,
  pub no_script: bool,
  pub verbose: bool,

}

impl Context {
  pub fn new(args: &ArgMatches) -> Context {
    let mut ctx = Context {
      yes: false,
      no_script: false,
      verbose: false,
    };

    ctx.set_yes(args.get_flag("yes"));
    ctx.set_verbose(args.get_flag("verbose"));

    return ctx;
  }

  pub fn set_yes(&mut self, yes: bool) {
    self.yes = yes;
  }

  pub fn set_no_script(&mut self, ns: bool) {
    self.no_script = ns;
  }

  pub fn set_verbose(&mut self, verbose: bool) {
    self.verbose = verbose
  }
}
