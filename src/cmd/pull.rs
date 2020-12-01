use clap_nested::Command;
use duct;

pub fn cmd<'a>() -> Command<'a, ()> {
    Command::new("pull")
        .description("Pull the latest docker images of services")
        .runner(|_env, _matches| {
            duct::cmd("docker-compose", vec!["pull"]).run().unwrap();
            Ok(())
        })
}
