# spruceid-gen-vc-demo

# Demo of a Simple CLI Tool to Generate Verifiable Credentials (VC)

Uses SpruceID's DIDKit to generate Verifiable Credentials[^1]

Verifiable Credentials are digital tamper-evident credentials that can be used
in a variety of use cases such as the digital versions of:

- driver's license
- passport
- diploma
- vaccination card
- concert ticket
- insurance card

## Arguments

The cli tool takes 3 arguments (in this order) which are all filepaths:

### unsigned_vc

- **_required_**
- A `.json` file containing the _unsigned Verifiable Credential_ payload

### key

> [!CAUTION] take proper precautions to not expose a sensitive private key!

- **_optional_**
- A `.jwk` file containing a key in _jwk_ format, including the private key
- if not provided, one will be generated

### out

> [!WARNING] The file will be overwritten if it exists!

- **_optional_**
- Path to dump the output, i.e., the _signed_ Verifiable Credential
- if not provided, stdout will be used

<hr />

## building & running

after cloning...

### development/local build & run

`cargo run -- {unsigned-vc.json} {key.jwk} {out.json}`

if using cargo, the arguments are provided after `--` (to let cargo know that,
"hey! these aren't for you! these are for the tool!")

In braces `{ ... }` are example filepaths that one might feed the tool

<br />

There's a `-h` flag to print the help text and `-v` for debug info.

<br />

Or, if you know what you're doing, please go for it.

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

If you see error about the `context`, and you're seeing a newer url defined in the _Verifiable_
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

[^1]:
    [https://www.w3.org/TR/vc-data-model-2.0/](https://www.w3.org/TR/vc-data-model-2.0/)

[^2]:
    [https://www.w3.org/TR/vc-data-model-2.0/#base-context](https://www.w3.org/TR/vc-data-model-2.0/#base-context)
