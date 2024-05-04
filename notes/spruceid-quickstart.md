###### Notes

# spruceid.dev Quickstart Documentation Re-Write

###### following commands can be found in the quickstart example on spruceid.dev[^1]

<br />

## install didkit-cli (using cargo)[^2]

`cargo install didkit-cli`

## generate a key

`didkit generate-ed25519-key`

ed25519 is an asymmetric key algorithm to generate a public/private key pair

The private key is used to sign data.

The public key is used to validate the signed output.

If the private key is kept private, anyone can use the public key to validate
that the data hasn't been tampered with because only the owner of the private
key could've signed it.

### outputs something like this (_prettified format for readability_):

```json
{
  "kty": "OKP",
  "crv": "Ed25519",
  "x": "Bta_JL8ddC1YVgRG8sdpv2PCFTM7b2wlgPZsWloCklI",
  "d": "6BX_QQnyo7wRTxkYU5LbXmQIewuLWBYhuzYlIlIjrfo"
}
```

> [!IMPORTANT] save it! this key is important!  
> It's used to sign the verifiable credential in order to ensure tampering
> hasn't occurred by validating the signed output with the same key
>
> e.g., write to a `.jwk` file for later use
> `didkit generate-ed25519-key > issuer_key.jwk`

## generate a DID key

`didkit key-to-did key -k <key>`

This command takes a <key> (like the one you saved as a `.jwk` file because...
you saved it, right?!?)

`issuer_did=$(didkit key-to-did key -k issuer_key.jwk)`

This uses the key in from the previous step saved in a `.jwk` file and also
stores it in a temporary variable `issuer_did` for easy use later

## (Optional) prepare a dummy unsigned VC (Verifiable Credential) if you don't already have one

This will generate a dummy unsigned VC:

```bash
cat > unsigned-vc.json <<EOF
{
  "@context": "https://www.w3.org/2018/credentials/v1",
    "id": "urn:uuid:`uuidgen`",
    "type": ["VerifiableCredential"],
    "issuer": "${issuer_did}",
    "issuanceDate": "$(date -u +%FT%TZ)",
    "credentialSubject": {
        "id": "did:example:my-data-subject-identifier"
    }
}
EOF
```

importantly, notice the DID is included as the `issuer` and the `type` is a
`["VerifiableCredential"]`

It should look something like this:

```json
{
  "@context": "https://www.w3.org/2018/credentials/v1",
  "id": "urn:uuid:1a87aaee-1238-4fa2-a99b-bda9f988bfa7",
  "type": ["VerifiableCredential"],
  "issuer": "did:key:z6MkeuxR1HoNqe45cw4cgjEXYBcV9yft1vZhj3v9jP1kJFa9",
  "issuanceDate": "2024-04-27T21:33:43Z",
  "credentialSubject": {
    "id": "did:example:my-data-subject-identifier"
  }
}
```

<br />
<br />
<br />

## if want to make arguments optional

### need to make sure one of these is true...

1. both cred file and jwk file are given
2. only jwk file is given
3. neither are given

```rust
let jwk: JWK;
match key {
    Some(key) => {
        let jwk_file_contents = fs::read_to_string(key)
            .expect("expected to be able to open the jwk file, please check and try again");
        jwk = serde_json::from_str(&jwk_file_contents).unwrap();
        println!("\nfrom file:\n{:#?}", jwk);
    }
    None => {
        jwk = JWK::generate_ed25519()?;
        println!("\ngenerated:\n{:#?}", jwk);
    }
}
```

<br />
<br />
<br />

```bash
# install cli
cargo install didkit-cli

# generate a key using ed25519
# this outputs in jwk format
# store in a .jwk file
# ***this key is important!***
# It's used to sign different pieces of data in order to ensure tampering
#   hasn't occurred by validating the signed output
didkit generate-ed25519-key > issuer_key.jwk

# contents of issuer_key.jwk should look like this (formatted for readability):
# {
# "kty": "OKP",
# "crv": "Ed25519",
# "x": "Bta_JL8ddC1YVgRG8sdpv2PCFTM7b2wlgPZsWloCklI",
# "d": "6BX_QQnyo7wRTxkYU5LbXmQIewuLWBYhuzYlIlIjrfo"
# }

#
# `key-to-did`
# generates the 'DID' with a provided key
#
# Use the key in our .jwk file generated in the previous step
#   and also stores it in a temporary variable for easy recall later
issuer_did=$(didkit key-to-did key -k issuer_key.jwk)

# print the generated DID
echo $issuer_did

# Create dummy VC data which we will sign to create the actual DID
# In JSON-LD format
# Some conveniences for dummy data:
#   `uuidgen`           UNIX cmd to generate a UUID
#                         (output is NOT *necessarily* cryptographically strong)
#   `date -u +%FT%TZ`   gets current timestamp in UTC
#   `issuer_did`        is the the DID created in the previous step.
cat > unsigned-vc.json <<EOF
{
  "@context": "https://www.w3.org/2018/credentials/v1",
    "id": "urn:uuid:`uuidgen`",
    "type": ["VerifiableCredential"],
    "issuer": "${issuer_did}",
    "issuanceDate": "$(date -u +%FT%TZ)",
    "credentialSubject": {
        "id": "did:example:my-data-subject-identifier"
    }
}
EOF

# Optionally, print contents of the file to confirm it looks as expected
cat unsigned-vc.json

# This generates the verification method that can be used to verify
#   the signed VC
# We store it in a temporary variable to use in the following command
vm=$(didkit key-to-verification-method key --key-path issuer_key.jwk)

#
# Finally, we get to the good part!
# This is what we've been working towards, i.e., our Verifiable Credential (VC)!
#

# `didkit vc-issue-credential` generates the VC
# Brief explanation of arguments used here:
#   key-path:             The key used for signing, or the path to the file
#                           containing the key as in our case
#   verification-method:  The method needed to verify the VC
#   proof-purpose:        The scope and intent of the generated VC
# We use the unsigned VC info (from a file) as input
# and output to another .json file
didkit vc-issue-credential --key-path issuer_key.jwk \
                           -v "${vm}" -p assertionMethod \
                           <unsigned-vc.json > signed-vc.json

# View output from previous command, i.e., which should be your signed VC
cat signed-vc.json
# The signed VC should look something like the following
# (*note: this has been slightly altered for easier readability):
#
# {
#   "@context": "https://www.w3.org/2018/credentials/v1",
#   "id": "urn:uuid:1a87aaee-1238-4fa2-a99b-bda9f988bfa7",
#   "type": [
#     "VerifiableCredential"
#   ],
#   "credentialSubject": {
#     "id": "did:example:my-data-subject-identifier"
#   },
#   "issuer": "did:key:z6MkeuxR1Ho...9jP1kJFa9",
#   "issuanceDate": "2024-04-27T21:33:43Z",
#   "proof": {
#     "type": "Ed25519Signature2018",
#     "proofPurpose": "assertionMethod",
#     "verificationMethod": "did:key:z6Mkeu...P1kJFa9",
#     "created": "2024-04-27T21:38:51.743410833Z",
#     "jws": "eyJ...mI2NCI6ZmFsc2V9..9XM...kTcp9a2qZbDh3LdKREk_DCQ"
#   }
# }
#
# the 2 '..' in the middle of the `jws`, not the 3 periods '...', should appear
#     in your output. A payload can go between the two, but there isn't one in
#     this case, so it's empty

# Now we can check that the VC can be validated with the `vc-verify-credential`
#     command of `didkit-cli`
didkit vc-verify-credential < signed-vc.json
# if valid, outputs:
# {"checks":["proof"],"warnings":[],"errors":[]}
```

[^1]:
    spruceid.dev didkit-cli quickstart <br />
    [https://www.spruceid.dev/quickstart](https://www.spruceid.dev/quickstart)

[^2]:
    didkit-cli on crates.io <br />
    [https://crates.io/crates/didkit-cli](https://crates.io/crates/didkit-cli)
