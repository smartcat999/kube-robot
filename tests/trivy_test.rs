use common::pkg::etc::config;
use common::pkg::etc::checker;
mod setup;

#[test]
fn test_config_check() -> anyhow::Result<()> {
    setup::setup();
    let mut conf = config::Config::default();
    conf.trivy.cache_dir = String::from("./tmp/cache");
    conf.trivy.reports_dir = String::from("./tmp/reports");

    assert_eq!(checker::check(conf)?, ());
    Ok(())
}