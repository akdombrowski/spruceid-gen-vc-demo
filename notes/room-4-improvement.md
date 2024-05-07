###### Notes

# Room for Improvements

## Libraries

### Invalid Context in VC

The library requires `https://www.w3.org/2018/credentials/v1` but the latest
spec explicitly requires `https://www.w3.org/ns/credentials/v2`

> @context
>
> The value of the @context property MUST be an ordered set where the first item
> is a URL with the value https://www.w3.org/ns/credentials/v2. For reference, a
> copy of the base context is provided in Appendix B.1 Base Context. Subsequent
> items in the array MUST be composed of any combination of URLs and/or objects
> where each is processable as a JSON-LD Context.

error isn't very descriptive saying the context is invalid at line 2, column 45

here's the unsigned vc that works just by swapping out the context value:

```json
{
  "@context": "https://www.w3.org/ns/credentials/v2",
  "id": "urn:uuid:1a87aaee-1238-4fa2-a99b-bda9f988bfa7",
  "type": ["VerifiableCredential"],
  "issuer": "did:key:z6MkeuxR1HoNqe45cw4cgjEXYBcV9yft1vZhj3v9jP1kJFa9",
  "issuanceDate": "2024-04-27T21:33:43Z",
  "credentialSubject": {
    "id": "did:example:my-data-subject-identifier"
  }
}
```

```sh
thread 'main' panicked at src/main.rs:134:6:
failed to generate proof: LDP(InconsistentProof(MissingAssociatedContext("https://www.w3.org/ns/credentials/issuer-dependent#Ed25519Signature2018")))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

even if I use the context it tells me is the missing associated context, it
fails:

```json
{
  "@context": "https://www.w3.org/ns/credentials/issuer-dependent#Ed25519Signature2018",
  "id": "urn:uuid:1a87aaee-1238-4fa2-a99b-bda9f988bfa7",
  "type": ["VerifiableCredential"],
  "issuer": "did:key:z6MkeuxR1HoNqe45cw4cgjEXYBcV9yft1vZhj3v9jP1kJFa9",
  "issuanceDate": "2024-04-27T21:33:43Z",
  "credentialSubject": {
    "id": "did:example:my-data-subject-identifier"
  }
}
```

```sh
thread 'main' panicked at src/main.rs:99:73:
called `Result::unwrap()` on an `Err` value: Error("Invalid context", line: 2, column: 87)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

or because it's difficult to get straight all of the different terminology in
this space to know, for example, difference between DID and VC (which I'm still
in the process of figuring out)

this fails as well with similarly unhelpful error

```json
{
  "@context": "https://www.w3.org/ns/did/v1",
  "id": "urn:uuid:1a87aaee-1238-4fa2-a99b-bda9f988bfa7",
  "type": ["VerifiableCredential"],
  "issuer": "did:key:z6MkeuxR1HoNqe45cw4cgjEXYBcV9yft1vZhj3v9jP1kJFa9",
  "issuanceDate": "2024-04-27T21:33:43Z",
  "credentialSubject": {
    "id": "did:example:my-data-subject-identifier"
  }
}
```

```sh
thread 'main' panicked at src/main.rs:99:73:
called `Result::unwrap()` on an `Err` value: Error("Invalid context", line: 2, column: 44)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

error for invalid json seems to come from here:

```rust
// ssi/ssi-json-ld/src/lib.rs

#[derive(Debug, thiserror::Error)]
pub enum ContextError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(#[from] json_syntax::parse::MetaError<Span>),

    #[error("Invalid JSON-LD context: {0}")]
    InvalidContext(#[from] Meta<json_ld::syntax::context::InvalidContext, Span>),
}


/// Parse a JSON-LD context.
pub fn parse_ld_context(content: &str) -> Result<RemoteContextReference, ContextError> {
    let json = json_syntax::Value::parse_str(content, |span| span)?;
    let context = json_ld::syntax::context::Value::try_from_json(json)?;
    Ok(RemoteContextReference::Loaded(RemoteContext::new(
        None, None, context,
    )))
}
```

```rust
// ssi/ssi-ldp/src/error.rs
#[derive(thiserror::Error, Debug)]
pub enum Error {
  ...
    #[error(transparent)]
    InvalidJsonLdContext(#[from] ssi_json_ld::ContextError),
  ...
}
```

### fix:

this _does_ work and is what's shown in the quickstart

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

for context, lines 98-99 read the file contents (containing the above) and
deserialize into a `Verifiable Credential`:

```rust
let cred_file = fs::read_to_string(cred);
let mut vc: VerifiableCredential = serde_json::from_str(&cred_file).unwrap();
let mut vc: VerifiableCredential = serde_json::from_str(&cred_file).unwrap();
```

and, lines 119-134 to generate the proof (using the VC from line 99 shown in the
code snippet above):

```rust
let mut context_loader = ContextLoader::default();
let ldp = LinkedDataProofOptions::default();
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
```

Mentioning that in the docs, more descriptive error messages, or a quick
primer/def'n on the context parameter, or a description of potential pitfalls in
the README.md for would help.

then do this:

    change the context url to `https://www.w3.org/2018/credentials/v1`

If you see this error, and you're seeing a newer url defined in the _Verifiable_
_Credentials Data Model v2.0_ spec[^1] as the base context (or mandated first
value) for `context`instead of `https://www.w3.org/2018/credentials/v1`, you
didn't necessarily misread the spec!

Because of the rapid development of the Verifiable Credentials space, the
associated specs like the _Verifiable Credentials Data Model v2.0_ are still
evolving as stated in the spec:

    The Working Group is expecting all of the terms and URLs supplied in the JSON-LD Context to be either stabilized, or removed, before the publication of this specification as a Proposed Recommendation.[^2]

Changes are inevitable and it may take a little bit before we're able to update
the library. If you notice this, then it'd help us and others out if you open a
pull request with the new change!

### CLI hangs when...

    didkit vc-issue-credential -k issuer_key.jwk

I believe it's waiting for stdin for the VC instead of failing because the
proper args weren't given

```rust
// didkit/cli/src/credential.rs

pub async fn issue(args: CredentialIssueArgs) -> Result<()> {
    let resolver = args.resolver_options.to_resolver();
    let mut context_loader = ContextLoader::default();
    // here
    let credential_reader = BufReader::new(stdin());
    //
    let mut credential: VerifiableCredential = serde_json::from_reader(credential_reader).unwrap();
    let proof_format = args.proof_options.proof_format.clone();
    let jwk_opt: Option<JWK> = args.key.get_jwk_opt();
    // ...
```

### Can't Generate JWT with Example Unsigned VC

I tested with a VC that was created using the one from the quickstart docs,
i.e., the `unsigned-vc.json` file it instructs you to create

this fails with error: `Error: UnencodableOptionClaim("checks")`

```rust
let unsigned_vc = fs::read_to_string("unsigned-vc.json")?;
let vc: VerifiableCredential = serde_json::from_str(&unsigned_vc).unwrap();

let did_resolver: &dyn DIDResolver = DID_METHODS.to_resolver();
let ldp = LinkedDataProofOptions::default();

// this fails with error:
// Error: UnencodableOptionClaim("checks")
let vc_unsigned_jwt = vc.generate_jwt(None, &ldp, did_resolver).await?;

println!("\n{:#?}", vc_unsigned_jwt);
```

[^1]:
    [https://www.w3.org/TR/vc-data-model-2.0/](https://www.w3.org/TR/vc-data-model-2.0/)

[^2]:
    [https://www.w3.org/TR/vc-data-model-2.0/#base-context](https://www.w3.org/TR/vc-data-model-2.0/#base-context)
