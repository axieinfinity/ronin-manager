mod install;
mod pull;
mod start;
mod stop;
mod upgrade;
mod wipe;

use clap_nested::Commander;

pub fn commander<'a>() -> Commander<'a, (), ()> {
    Commander::new()
        .options(|app| {
            app.name(clap::crate_name!())
                .version(clap::crate_version!())
                .about(clap::crate_description!())
                .author(clap::crate_authors!())
        })
        .add_cmd(install::cmd())
        .add_cmd(start::cmd())
        .add_cmd(stop::cmd())
        .add_cmd(pull::cmd())
        .add_cmd(upgrade::cmd())
        .add_cmd(wipe::cmd())
        .no_cmd(|_args, _matches| {
            println!("No subcommand matched");
            Ok(())
        })
}
