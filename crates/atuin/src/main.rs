#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::use_self, clippy::missing_const_for_fn)] // not 100% reliable

use clap::Parser;
use eyre::Result;

use command::AtuinCmd;

mod command;

#[cfg(feature = "sync")]
mod sync;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const SHA: &str = env!("GIT_HASH");

static HELP_TEMPLATE: &str = "\
{before-help}{name} {version}
{author}
{about}

{usage-heading}
  {usage}

{all-args}{after-help}";

/// Magical shell history
#[derive(Parser)]
#[command(
    author = "Ellie Huxtable <ellie@atuin.sh>",
    version = VERSION,
    help_template(HELP_TEMPLATE),
)]
struct Atuin {
    #[command(subcommand)]
    atuin: AtuinCmd,
}

impl Atuin {
    fn run(self) -> Result<()> {
        self.atuin.run()
    }
}

fn main() -> Result<()> {
    eprintln!("autin running");
    eprintln!("getuid() => {:#?}", nix::unistd::getuid());
    eprintln!("getgid() => {:#?}", nix::unistd::getgid());
    eprintln!("geteuid() => {:#?}", nix::unistd::geteuid());
    eprintln!("getegid() => {:#?}", nix::unistd::getegid());
    eprintln!("getresuid() => {:#?}", nix::unistd::getresuid());
    eprintln!("getresgid() => {:#?}", nix::unistd::getresgid());
    eprintln!("getgroups() => {:#?}", nix::unistd::getgroups());
    eprintln!("getpgid() => {:#?}", nix::unistd::getpgid(Some(nix::unistd::Pid::this())));
    eprintln!("getpgrp() => {:#?}", nix::unistd::getpgrp());
    eprintln!("getpid() => {:#?}", nix::unistd::getpid());
    eprintln!("getppid() => {:#?}", nix::unistd::getppid());
    eprintln!("getsid() => {:#?}", nix::unistd::getsid(Some(nix::unistd::Pid::from_raw(0))));
    eprintln!("gettid() => {:#?}", nix::unistd::gettid());
    // eprintln!("now sleeping, inspect me!");
    // std::thread::sleep(std::time::Duration::from_secs(900));

    Atuin::parse().run()
}
