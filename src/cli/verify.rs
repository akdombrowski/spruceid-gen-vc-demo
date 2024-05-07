use didkit::{
    ContextLoader, DIDResolver, LinkedDataProofOptions, VerifiableCredential, DID_METHODS, JWK,
};
use std::ffi::OsString;
use std::fs;

use crate::cli::CliArgs;

// Read signed VC from file and use a jwk from file to verify. Optionally, print
// verification result to file, else print to stdout.
pub async fn verify_signed_vc(
    args: &CliArgs,
    out: &Option<OsString>,
    debug: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let CliArgs { cred, key } = args;

    let cred_file = fs::read_to_string(cred)
        .expect("expected to be able to open the unsigned VC file, please check the filepath provided and try again");
    let vc: VerifiableCredential = serde_json::from_str(&cred_file).unwrap();

    if debug {
        println!("\nvc from file:\n{:#?}", vc);
    }

    // Basic check that a 'valid' (not 'verified') VC was provided
    vc.validate()?;

    if debug {
        println!("vc passed 'validation' check (not to be confused with 'verification')");
    }

    // Read key from provided file path. This needs to be the same key that
    // was used to sign the VC otherwise verification will fail.
    let jwk_file_contents = fs::read_to_string(key)
    .expect("expected to be able to open the jwk file, please check the filepath provided and try again");
    let jwk: JWK = serde_json::from_str(&jwk_file_contents)?;

    if debug {
        println!("\njwk from file:\n{:#?}", jwk);
    }

    let mut context_loader = ContextLoader::default();
    let did_resolver: &dyn DIDResolver = DID_METHODS.to_resolver();
    let ldp = LinkedDataProofOptions::default();

    let verify_result = vc
        .verify(Some(ldp), did_resolver, &mut context_loader)
        .await;

    if debug {
        println!("verified?  {:#?}", verify_result);
    }

    // If an out_path argument is provided,
    //    write the VC to that file,
    // else print to stdout

    // Convert verification result to string to write to file or to stdout
    let de_verify_result = serde_json::to_string(&verify_result)
        .expect("expected the final signed verification result to be serializable");
    match out {
        Some(out) => {
            // !!This is a bit unsafe writing to a file at whatever path the user
            // provided without first checking if we really should...
            println!("\nwriting to '{}'...", out.to_str().unwrap());

            if debug {
                println!("\n{}", de_verify_result)
            }

            fs::write(out, de_verify_result)?;

            println!("\ndone!\n")
        }
        None => {
            // to stdout
            println!("\n{de_verify_result}")
        }
    }

    Ok(())
}
