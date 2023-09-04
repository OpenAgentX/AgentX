#[warn(unused_imports)]

use anyhow::Result;
use clap::Parser;
use tracing::{info, Level};
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
    /// Your innovative task, such as 'Creating a snake game.'
    #[arg(short, long)]
    idea: String,
    /// Investment amount
    #[arg(short, long, default_value_t = 3.0)]
    startup_investment: f64,
    /// Number of startup rounds
    #[arg(short, long, default_value_t = 4)]
    n_round: i32,
    /// Enable code review
    #[arg(short, long)]
    code_review: bool,
    /// Run tests during development
    #[arg(short, long)]
    run_tests: bool,
}

#[tokio::main]
async fn main() {


    tracing_subscriber::fmt()
        // enable everything
        .with_max_level(tracing::Level::INFO)
        .with_timer(time::LocalTime::rfc_3339())
        // sets this to be the default, global collector for this application.
        .init();

    let args = Args::parse();
    info!("Hello {}!", args.idea);


    let _ = startup(args.idea, args.startup_investment, args.n_round, args.code_review, args.run_tests).await;
}
