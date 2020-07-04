fn main() {
    env_logger::from_env(env_logger::Env::new().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    epm::cmds::exec();
}
