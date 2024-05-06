pub mod Cli {
    use std::ffi::{OsStr, OsString};

    use clap::{Args, Parser, Subcommand, ValueHint};

    pub mod generate;
    pub mod verify;

    #[derive(Parser, Debug)]
    #[command(name = "spruceid-gen-vc-demo")]
    #[command(version, propagate_version = true)]
    #[command(
        about = "DIDKit-based VC generator and verifier",
        long_about = "A Verification Credential generator and verifier using SpruceID's DIDKit"
    )]
    #[command(next_line_help = true)]
    pub struct Cli_Parser {
        // /// path to a .json file containing the unsigned VC
        // #[arg(value_name = "VC")]
        // #[arg(value_hint = ValueHint::FilePath)]
        // #[arg(required = true)]
        // cred: Option<OsString>,

        // /// path to the .jwk file containing the signing key
        // /// *Caution: this will overwrite a file that already exists, else it will
        // /// create the file and write the VC there
        // #[arg(value_name = "PUBLIC_KEY")]
        // #[arg(value_hint = ValueHint::FilePath)]
        // key: Option<OsString>,
        //
        /// path to the file to send the output
        #[arg(value_name = "OUT")]
        #[arg(value_hint = ValueHint::FilePath)]
        #[arg(global = true)]
        #[arg(required = false)]
        pub out: Option<OsString>,

        /// Turn debugging information on
        #[arg(short = 'v', long = "verbose")]
        #[arg(global = true)]
        pub debug: bool,

        #[command(subcommand)]
        pub command: Commands,
    }

    #[derive(Debug, Subcommand)]
    #[command(flatten_help = true)]
    pub enum Commands {
        /// Generates a signed Verifiable Credential
        #[command(arg_required_else_help = true)]
        #[command(aliases=["gen", "sign", "gen-vc"])]
        #[command(name = "generate")]
        GenerateSignedVC(GenerateArgs),

        /// Generates a signed Verifiable Credential
        #[command(arg_required_else_help = true)]
        #[command(aliases=["verifiable", "verify-signed", "check"])]
        #[command(name = "verify")]
        VerifySignedVC(VerifyArgs),
    }

    // #[command(next_line_help = true)]
    // #[command(args_conflicts_with_subcommands = true)]
    #[derive(Args, Debug)]
    pub struct GenerateArgs {
        /// path to a .json file containing the unsigned VC
        #[arg(value_name = "VERIFIABLE_CREDENTIAL_FILE")]
        pub cred: OsString,

        /// path to the .jwk file containing the key to use for verification
        #[arg(value_name = "KEY_FILE")]
        pub key: OsString,
    }

    #[derive(Args, Debug)]
    // #[command(next_line_help = true)]
    // #[command(args_conflicts_with_subcommands = true)]
    pub struct VerifyArgs {
        /// path to a .json file containing the signed VC
        #[arg(value_name = "SIGNED_VC")]
        #[arg(value_hint = ValueHint::FilePath)]
        #[arg(requires = "key")]
        pub cred: OsString,

        /// path to the .jwk file containing the key to use for verification
        #[arg(value_name = "PRIVATE_KEY")]
        #[arg(requires = "cred")]
        pub key: OsString,
    }

    pub async fn run(args: &Cli_Parser) -> Result<(), Box<dyn std::error::Error>> {
        // cred and key are required
        // out is optional

        match &args.command {
            Commands::GenerateSignedVC(gen_args) => {
                let GenerateArgs { cred, key } = &gen_args;

                println!("\ngen_args: \n{:#?}", gen_args);
                println!("\ncred file: {:#?}", cred);
                println!("key file: {:#?}\n", key);

                generate::generate_signed_vc(gen_args, &args.out, args.debug).await?;
            }

            Commands::VerifySignedVC(verify_args) => {
                let VerifyArgs { cred, key } = &verify_args;

                println!("\ngen_args: \n{:#?}", verify_args);
                println!("\ncred file: {:#?}", cred);
                println!("key file: {:#?}\n", key);

                verify::verify_signed_vc(verify_args, &args.out, args.debug).await?;
            }
        }

        Ok(())
    }
}
