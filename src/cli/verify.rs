use didkit::{
    ContextLoader, DIDResolver, LinkedDataProofOptions, VerifiableCredential, DID_METHODS, JWK,
};
use std::ffi::{OsStr, OsString};
use std::fs;

use crate::cli::SignedVCArgs;

pub async fn verify_signed_vc(
    args: &SignedVCArgs,
    out: &Option<OsString>,
    debug: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let SignedVCArgs { cred, key } = args;

    // If an out_path argument is provided,
    //    write the VC to that file,
    // else print to stdout
    // let de_vc =
    //     serde_json::to_string(&vc).expect("expected the final signed VC to be serializable");
    // match out {
    //     Some(out) => {
    //         // !!This is a bit unsafe writing to a file at whatever path the user
    //         // provided without first checking if we really should...
    //         println!("\nwriting to '{}'...", out.to_str().unwrap());

    //         if debug {
    //             println!("\n{}", de_vc)
    //         }

    //         fs::write(out, de_vc)?;
    //         println!("\ndone!\n")
    //     }
    //     None => {
    //         // to stdout
    //         println!("\n{de_vc}")
    //     }
    // }

    Ok(())
}
