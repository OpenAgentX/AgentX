
use tracing_subscriber::fmt::time;
use agent_roles::{AgentRoleBuilder, ResearchAgent};
use anyhow::Result;
use clap::Parser;
use clap::builder::TypedValueParser as _;
use dotenv::dotenv;

#[derive(clap::ValueEnum, Debug, Clone)] // ArgEnum here
// #[clap(rename_all = "report_type")]
pub enum ReportTpye {
    Research,
    Resource,
    Outline,
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
    task: String,
    #[arg(short, long)]
    #[clap(value_enum)]
    report_type: ReportTpye,
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
async fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();
    tracing_subscriber::fmt()
        // enable everything
        .with_max_level(args.log_level)
        .with_timer(time::LocalTime::rfc_3339())
        // sets this to be the default, global collector for this application.
        .init();

    let builder = AgentRoleBuilder::default();

    let task = args.task;

    let agent = builder.choose_agent(&task).await;

    let mut ra = ResearchAgent::new("gpt_researcher", &agent.agent, &agent.agent_role_prompt, "", "");
    ra.conduct_research(&task).await;


    let report_type = match args.report_type {
        ReportTpye::Research => "research_report",
        ReportTpye::Resource => "resource_report",
        ReportTpye::Outline => "outline_report",
    };

    let _res = ra.write_report(report_type).await;
    Ok(())
}
