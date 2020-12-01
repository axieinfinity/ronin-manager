use clap_nested::Command;
use duct;

pub fn cmd<'a>() -> Command<'a, ()> {
    Command::new("start")
        .description("Start all services")
        .runner(|_env, _matches| {
            duct::cmd(
                "docker-compose",
                vec!["--env-file", "~/.skymavis/.env", "up", "-d"],
            )
            .run()
            .unwrap();
            Ok(())
        })
}
