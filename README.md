<hr />
<hr />
While not yet complete, if interested, you can find more of my dev experience
notes and feedback in the 
[notes folder](./notes/)

- What's likely difficult for a newcomer to digital credentials to get up to
  speed quickly and what kinds of things can help[^1]

- Areas where I hit issues or problems or unclear info off opportunities for
  improvement[^2]
- A partial rework/supplement of the Quickstart on SpruceID.dev[^3]
<hr />
<hr />

# spruceid-gen-vc-demo

# Demo of a Simple CLI Tool to Generate Verifiable Credentials (VC)

Uses SpruceID's DIDKit[^6] to generate Verifiable Credentials[^4]

Verifiable Credentials are digital tamper-evident credentials that can be used
in a variety of use cases such as the digital versions of:

- driver's license
- passport
- diploma
- vaccination card
- concert ticket
- insurance card

<details>
<summary>CLI Help</summary>

Use with `-h` to see the help reference:

```sh
$ spruceid-gen-vc-demo -h
DIDKit-based VC generator and verifier

Usage: spruceid-gen-vc-demo generate [OPTIONS] <VERIFIABLE_CREDENTIAL_FILE> <KEY_FILE> [OUT]
       spruceid-gen-vc-demo verify [OPTIONS] <VERIFIABLE_CREDENTIAL_FILE> <KEY_FILE> [OUT]
       spruceid-gen-vc-demo help [COMMAND]...

Arguments:
  [OUT]
          path to the file to send the output

Options:
  -v, --verbose
          Turn debugging information on
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version

spruceid-gen-vc-demo generate:
Generates a signed Verifiable Credential
  -h, --help
          Print help
  -V, --version
          Print version
  <VERIFIABLE_CREDENTIAL_FILE>
          path to a .json file containing the unsigned VC
  <KEY_FILE>
          path to the .jwk file containing the key to use for verification

spruceid-gen-vc-demo verify:
Generates a signed Verifiable Credential
  -h, --help
          Print help
  -V, --version
          Print version
  <VERIFIABLE_CREDENTIAL_FILE>
          path to a .json file containing the unsigned VC
  <KEY_FILE>
          path to the .jwk file containing the key to use for verification

spruceid-gen-vc-demo help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...
          Print help for the subcommand(s)
```

</details>

## Arguments

Both (sub)commands take the same arguments requiring two separate filepaths for
a VC and a JWK.

Optionally, an additional filepath can be provided to write the output to.

The order is important! From first to last:

1. vc_path
2. jwk_path
3. (optional) output_path

> [!CAUTION]
>
> Take proper precautions to not expose a sensitive private key!

## (Sub)Commands

### _generate_

Generates a signed VC from an unsigned one and a JWK.

- The VC provided should be an unsigned VC.

- The JWK should be the key to be used to sign the provided unsigned VC.

- (Optional) output path to write to file instead of stdout

Example (test files can be found in [examples/](examples/)):

```sh
spruceid-gen-vc-demo gen examples/unsigned-vc.json examples/issuer-key.jwk

{"@context":"https://www.w3.org/2018/credentials/v1","id":"urn:uuid:1a87aaee-1238-4fa2-a99b-bda9f988bfa7","type":["VerifiableCredential"],"credentialSubject":{"id":"did:example:my-data-subject-identifier"},"issuer":"did:key:z6MkeuxR1HoNqe45cw4cgjEXYBcV9yft1vZhj3v9jP1kJFa9","issuanceDate":"2024-04-27T21:33:43Z","proof":{"type":"Ed25519Signature2018","proofPurpose":"assertionMethod","verificationMethod":"did:key:z6MkeuxR1HoNqe45cw4cgjEXYBcV9yft1vZhj3v9jP1kJFa9#z6MkeuxR1HoNqe45cw4cgjEXYBcV9yft1vZhj3v9jP1kJFa9","created":"2024-05-07T14:21:52.874463529Z","jws":"eyJhbGciOiJFZERTQSIsImNyaXQiOlsiYjY0Il0sImI2NCI6ZmFsc2V9..u59OgKOXa-ARBtrnrH3JDgNL8G95O4QMLFGAX1MkNaES5UsXDMSoRCymo9RxpKO5ZLrHZJRmWR67cYNjUMc7DA"}}
```

### _verify_

Verifies a signed VC with a provided JWK.

- The VC provided should be a signed VC.

- The JWK should be the key that was used to sign the signed VC.

- (Optional) output path to write to file instead of stdout

Example (test files can be found in [examples/](examples/)):

```sh
spruceid-gen-vc-demo verify examples/signed-vc.json examples/issuer-key.jwk

{"checks":["proof"],"warnings":[],"errors":[]}
```

- **_optional_**
- A `.jwk` file containing a key in _jwk_ format, including the private key
- if not provided, one will be generated

### out

> [!WARNING]
>
> The file will be overwritten if it exists!

- **_optional_**
- Path to dump the output, i.e., the _signed_ Verifiable Credential
- if not provided, stdout will be used

<hr />

## building & running

The binary is compiled in the root dir as './spruceid-gen-vc-demo'.

If it doesn't work on your machine or you want to play around with the source
code, you'll need to use `cargo run -- ` or recompile.

after cloning...

### development/local build & run

`cargo run -- {vc.json} {key.jwk} {out.json}`

if using cargo, the arguments are provided after `--` (to let cargo know that,
"hey! these aren't for you! these are for the tool!")

In braces `{ ... }` are example filepaths that one might feed the tool

<br />

There's a `-h` flag to print the help text and `-v` for debug info.

<br />

Or, if you know what you're doing, please go for it! 😊

<br />

    I would typically do something like this below for any library/sample published (this is just a start)...

## Potential Errors and Fixes

### error:

if you see this:

```sh
failed to generate proof: LDP(InconsistentProof(MissingAssociatedContext("https://www.w3.org/ns/credentials/issuer-dependent#Ed25519Signature2018")))
```

or this:

```sh
called `Result::unwrap()` on an `Err` value: Error("Invalid context", line: 2, column: 45)
```

- note that your line and column numbers might be slightly different depending
  on your formatting, but if it's complaining about "invalid" or
  "missingassociated" `context` property, try the fix.

### fix:

then do this:

    change the context url to `https://www.w3.org/2018/credentials/v1`

You may see this error even if you see a newer url defined in the _Verifiable
Credentials Data Model v2.0_ spec[^4] as the required base context (or mandated
first value) for `context`instead of `https://www.w3.org/2018/credentials/v1`.
No, you didn't necessarily misread the spec!

Because of the rapid development of the Verifiable Credentials space, the
associated specs like the _Verifiable Credentials Data Model v2.0_ are still
evolving as stated in the spec[^5]:

    The Working Group is expecting all of the terms and URLs supplied in the JSON-LD Context to be either stabilized, or removed, before the publication of this specification as a Proposed Recommendation.

Changes are inevitable and it may take a little bit before we're able to update
the library. If you notice this, then it'd help us and others out if you open a
pull request with the new change!

[^1]: [notes/quickHitLessons](notes/quickHitLessons)
[^2]: [notes/room-4-improvements](notes/room-4-improvements)
[^3]: [notes/spruceid-quickstart](notes/spruceid-quickstart)
[^4]:
    [https://www.w3.org/TR/vc-data-model-2.0/](https://www.w3.org/TR/vc-data-model-2.0/)

[^5]:
    [https://www.w3.org/TR/vc-data-model-2.0/#base-context](https://www.w3.org/TR/vc-data-model-2.0/#base-context)

[^6]: [https://github.com/spruceid/didkit](https://github.com/spruceid/didkit)
