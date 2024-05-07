use clap::Parser;

use spruceid_gen_vc_demo::cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // consume user's cli input
    let args = cli::CliParser::parse();

    // Calls cli run to start processing with supplied input args (incl. cmd)
    cli::run(&args).await?;

    Ok(())
}
