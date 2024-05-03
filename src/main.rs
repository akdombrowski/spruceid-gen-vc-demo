use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;

use clap::{Parser, ValueHint};

use didkit::{
    ContextLoader, DIDResolver, LinkedDataProofOptions, VerifiableCredential, DID_METHODS, JWK,
};

#[derive(Parser, Debug)]
#[command(name = "spruceid-gen-vc-demo")]
#[command(version, propagate_version = true)]
#[command(
    about = "DIDKit-based VC generator and verifier",
    long_about = "A Verification Credential generator and verifier using SpruceID's DIDKit"
)]
#[command(next_line_help = true)]
struct Cli {
    /// path to a .json file containing the unsigned VC
    #[arg(value_name = "UNSIGNED_VC")]
    #[arg(value_hint = ValueHint::FilePath)]
    cred: Option<OsString>,

    /// path to the .jwk file containing the signing key
    /// *Caution: this will overwrite a file that already exists, else it will
    /// create the file and write the VC there
    #[arg(value_name = "PUBLIC_KEY")]
    #[arg(value_hint = ValueHint::FilePath)]
    key: Option<OsString>,

    /// path to the file to send the output
    #[arg(value_name = "OUT_FILE")]
    #[arg(value_hint = ValueHint::FilePath)]
    out_file: Option<OsString>,

    /// Turn debugging information on
    #[arg(short = 'v', long = "verbose")]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // cred and key are required
    // out is optional
    let cred: &OsStr = args
        .cred
        .as_deref()
        .expect("expected a path to a .json file containing an unsigned VC as the first argument");
    let key: &OsStr = args
        .key
        .as_deref()
        .expect("expected a path to a .jwk file containing a key as the second argument");
    let out: Option<&OsStr> = args.out_file.as_deref();

    // didkit-cli quickstart equivalent steps to generate a VC

    // 1. `didkit generate-ed25519-key`
    // 2. `didkit key-to-did key -k <key>`
    // 3. `didkit key-to-verification-method key --key-path issuer_key.jwk`
    // 4. `didkit vc-issue-credential --key-path issuer_key.jwk -v <did-key> -p
    //        assertionMethod <unsigned-vc.json > signed-vc.json`

    // 1. `didkit generate-ed25519-key`

    // Need to use the same key that generated the Issuer didkey in the unsigned
    // VC, else later steps (generate proof, for one) will fail

    // In the cli quickstart, it first instructs you to create a new key, but
    // this allows the user to supply their own in a .jwk file
    //
    // Could move to
    // if user supplies own jwk file:
    //    read from file and serialize into json
    // else
    //    can generate one for the user:
    //
    let jwk_file_contents = fs::read_to_string(key)
    .expect("expected to be able to open the jwk file, please check the filepath provided and try again");
    let jwk: JWK = serde_json::from_str(&jwk_file_contents).unwrap();

    if args.debug {
        println!("\nfrom file:\n{:#?}", jwk);
    }

    // 2. `didkit key-to-did key -k <key>`
    // 3. `didkit key-to-verification-method key --key-path issuer_key.jwk`

    // The equivalent quickstart step is creating the did-key and using it to
    // create the unsigned-vc.json file
    //
    // Instead, here we read an already created 'unsigned-vc.json'-type
    // file the user has provided us the path to
    let did_resolver: &dyn DIDResolver = DID_METHODS.to_resolver();
    let cred_file = fs::read_to_string(cred)
        .expect("expected to be able to open the unsigned VC file, please check the filepath provided and try again");
    let mut vc: VerifiableCredential = serde_json::from_str(&cred_file).unwrap();

    if args.debug {
        println!("\nfrom file:\n{:#?}", vc);
    }

    // Check if the provided unsigned VC is valid (different from verified)
    vc.validate_unsigned().expect("expected the unsigned VC to be valid, please check the contents of the file and the error, then try again");

    if args.debug {
        println!(
            "\nvalid unsigned VC? {:#?}",
            vc.validate_unsigned().and(Ok(true))?
        );
    }

    // 4. `didkit vc-issue-credential --key-path issuer_key.jwk -v
    //    <verification_method> -p "assertionMethod" <unsigned-vc.json

    // Using defaults for context loader and LinkedDataProofOptions
    let mut context_loader = ContextLoader::default();
    let ldp = LinkedDataProofOptions::default();

    // This fn will error with 'keymismatch' when the issuer in the unsigned VC
    // has a different didkey than expected.  Make sure you use the same key as
    // in the unsigned VC file as the `jwk` used here.
    let proof = didkit::generate_proof(
        &vc,
        Option::from(jwk).as_ref(),
        ldp,
        did_resolver,
        &mut context_loader,
        None,
    )
    .await
    .expect("failed to generate proof");

    if args.debug {
        println!("\n{:#?}", proof);
    }

    // Add the proof to the unsigned VC, effectively turning it into a signed VC
    vc.add_proof(proof);

    // If an out_path argument is provided,
    //    write the VC to that file,
    // else print to stdout
    let de_vc =
        serde_json::to_string(&vc).expect("expected the final signed VC to be serializable");
    match out {
        Some(out) => {
            // !!This is a bit unsafe writing to a file at whatever path the user
            // provided without first checking if we really should...
            println!("\nwriting to '{}'...", out.to_str().unwrap());

            if args.debug {
                println!("\n{}", de_vc)
            }

            fs::write(out, de_vc)?;
            println!("\ndone!\n")
        }
        None => {
            // to stdout
            println!("\n{de_vc}")
        }
    }
    Ok(())
}
