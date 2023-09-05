#[warn(unused_imports)]
use std::path::PathBuf;

use anyhow::Result;
use clap::builder::TypedValueParser as _;
use clap::Parser;

use tracing::info;
use tracing_subscriber::fmt::time;

use agentx_core::SoftwareCompany;

async fn startup(
    idea: String,
    _investment: f64,
    n_round: i32,
    _code_review: bool,
    _run_tests: bool,
) -> Result<()> {
    

    let cfg = "config/key.yaml";
    let mut company = SoftwareCompany::new(cfg);
    // let mut env = Environment::new();

    company.hire(vec![
        Box::new(agent_roles::ProductManager::default()),
        Box::new(agent_roles::Architect::default()),
        Box::new(agent_roles::ProjectManager::default()),
        Box::new(agent_roles::Engineer::default()),
    ]);

    let investment = 3.0;
    
    company.invest(investment);
    company.start_project(&idea);
    company.run(n_round).await;
    Ok(())
}


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name="AgentX", author="sxhxliang", version="0.1.0",
    about="Assign different roles to GPTs to form a collaborative software entity for complex tasks.",
    long_about=None
)]
struct Args {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    /// Your innovative task, such as 'Creating a snake game.'
    #[arg(short, long)]
    idea: String,
    /// Agent Name
    #[arg(short, long, default_value_t = String::from("MetaGPT"))]
    agent: String,
    /// Investment amount
    #[arg(short, long, default_value_t = 3.0)]
    startup_investment: f64,
    /// Number of startup rounds
    #[arg(short, long, default_value_t = 4)]
    n_round: i32,
    /// Enable code review
    #[arg(short, long)]
    review: bool,
    /// Run tests during development
    #[arg(short, long)]
    tests: bool,
    /// Support enums from a foreign crate that don't implement `ValueEnum`
    #[arg(
        short,
        long,
        default_value_t = tracing::Level::INFO,
        value_parser = clap::builder::PossibleValuesParser::new(["TRACE", "DEBUG", "INFO", "WARN", "ERROR"])
            .map(|s| s.parse::<tracing::Level>().unwrap()),
    )]
    log_level: tracing::Level,
}

#[tokio::main]
async fn main() {

    let args = Args::parse();

    tracing_subscriber::fmt()
        // enable everything
        .with_max_level(args.log_level)
        .with_timer(time::LocalTime::rfc_3339())
        // sets this to be the default, global collector for this application.
        .init();

    info!("Hello, use {} for {}!", args.agent, args.idea);

    let _ = startup(args.idea, args.startup_investment, args.n_round, args.review, args.tests).await;
}
