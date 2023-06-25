# URN: Unidimensional Rewriting Notation

## Big Idea
URN is a notation system designed to let you specify a simple Abstract Rewriting System ( ARS ) so that it can then be automatically step-wise rewritten.


## Getting Started
If you want to get started you'll need one of two things! [Nix ( w/ flakes! )](https://nixos.org/) or [Rust](https://www.rust-lang.org/) ( unless you are willing to re-implement URN according to the specification which... be my guest! )
Follow the instructions below depending on the option you choose.

To start learning URN head over to [examples](./examples) and try running some of them.

---

### With Nix
1. Clone this Repo
```
git clone git@github.com:khora-seule/urn.git
```
2. Setup Development Enviroment
```
nix develop
```
3. Build URN
```
cargo build
```
4. Run URN
```
./target/debug/urn ~/.../terms.urn ~/.../rules.urn
```

---

### With Rust
1. Clone this Repo
```
git clone git@github.com:khora-seule/urn.git
```
2. Build URN
```
cargo build
```
3. Run URN
```
./target/debug/urn ~/.../terms.urn ~/.../rules.urn
```

---

## Terms & Rules
There are two constructs in URN: `Terms` and `Rules`. 
- `Terms` are either strings delimited by whitespace or are delimited by brackets -- `{` or `}` -- and have `Terms` nested inside them. All the `Terms` for a given ARS are contained in one `Terms` file. 
- `Rules` are pairs of `Terms` and represent an opportunity for the first `Term` to be rewritten as the second `Term`. All `Rules` for a given ARS are contained in one `Rules` file; they are paired implicitly with no additional notation -- this means you must take care to make sure your `Rules` file is valid ( has an even number of `terms` ) or you will encounter an error.

---

## Rewriting Using Rules
- Rewriting `Terms` using `Rules` follows an implicit precedence in order to ensure that rewriting is always deterministic -- however, the addition of a flag for nondeterministic rewriting is planned -- though no other guarantees are currently provided, e.g. termination. 
- The Rewriting proccess occurs as follows:
> 0. ` n = 1 `
> 1. All `terms` which appear as the `Pre-Term` of the nth `Rule` are rewritten into the `Post-Term`.
> 2. If there are none, then the first step is repeated with ` n += 1 `.
> 3. As soon as any `terms` are rewritten, start over from the first step with ` n = 1 `.
> 4. If all `Rules` fail to result in a rewrite then the ARS has terminated.

---

## Important Note About Release 0.1
As of the initial release, URN only works in an obvious way for `term` files containing only one `term`. This is because scope is purely structural in URN, so all "top-level" `terms` behave according to extrinsic equality -- including during rewriting. Since only one Rewrite step is occuring at a time, and scope is only structural, if one top-level `term` is rewritten to be identical to another current top-level `term`, then URN proceeds to ignore the multiplicity, since the rewrite will be identical for both `terms`.
This means that -- at the moment -- in order to have URN behave as expected, it will need to be the case that all of the rewrite paths of each `term` in a `term` file are disjoint i.e. none of the rewrite paths have any intermediate `terms` in common.
This limitation isn't ideal for several reasons, and the implementation -- and likely specification -- of the language will change to enable multiple `terms` in future versions. Likely the following 'major' version will support this.