##Specification for this
#**U***nidimensional* **R***ewriting* **N***otation*

- Big Idea
> URN is a notation system designed to let you specify a simple ARS so that it can then be automatically maximally rewritten.

- Terms & Rules
> - There are two constructs in URN: `Terms` and `Rules`. 
> - `Terms` are either strings delimited by whitespace or are delimited by brackets -- `{` or `}` -- and have `Terms` nested inside them. All the `Terms` for a given ARS are contained in one `Terms` file. 
> - `Rules` are pairs of `Terms` and represent an opportunity for the first `Term` to be rewritten as the second `Term`. All `Rules` for a given ARS are contained in one `Rules` file.

- Rewriting Using Rules
> - Rewriting `Terms` using `Rules` follows an implicit precedence in order to ensure that rewriting is always deterministic ( though no other guarantees are provided ). 
- Rewriting occurs as follows:
> 1. All `terms` which appear on the `LHS` of the nth `Rule` are rewritten.
> 2. If there are none, then the first step is repeated with the next `Rule`.
> 3. As soon as any `terms` are rewritten, start over from the first step with `n = 1`.
> 4. If all `Rules` fail to result in a rewrite then the ARS has terminated.