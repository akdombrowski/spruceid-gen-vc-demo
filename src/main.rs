use clap::Parser;

use spruceid_gen_vc_demo::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::Cli_Parser::parse();

    // let cred = args.gen_args.cred.to_os_string();
    // let key = args.gen_args.key.to_os_string();
    // let out = args.out.as_deref();

    // let gen_args = GenerateArgs { cred, key };

    // let _gen_vc = generate(&gen_args, out, args.debug);

    Cli::run(&args).await?;

    Ok(())
}
