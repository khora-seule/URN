# Boolean Example in URN

For our Boolean example we have a `terms.urn` that looks like this:
```
{
    {
        { True or True } and { False or False }
    }
    xor
    {
        { True and True } or { not { False and False } }
    }
}
```

And we have a `rules.urn`that looks like this:
```
{ False and False } { False }
{ False and True }  { False }
{ True and False }  { False }
{ True and True }   { True }

{ False or False }  { False }
{ False or True }   { True }
{ True or False }   { True }
{ True or True }    { True }

{ False xor False }  { False }
{ False xor True }   { True }
{ True xor False }   { True }
{ True xor True }    { False }

{ not True }    { False }
{ not False }   { True }

{ { True } }    { True }
{ { False } }   { False }

{ { False } and { False } } { False }
{ { False } and { True } }  { False }
{ { True } and { False } }  { False }
{ { True } and { True } }   { True }

{ { False } or { False } }  { False }
{ { False } or { True } }   { True }
{ { True } or { False } }   { True }
{ { True } or { True } }    { True }

{ { False } xor { False } } { False }
{ { False } xor { True } }  { True }
{ { True } xor { False } }  { True }
{ { True } xor { True } }   { False }

{ not { True } }    { False }
{ not { False } }   { True }
```
## Reading the Rules
We can read the rules off of `rules.urn` line by line, since each line has exactly two terms. We'll go block by block -- indicated by full line-breaks -- and give a brief description of the `rules` in aggregate.
- The first three blocks of four lines are functionally operation tables for boolean `And`, `Or` and `Xor` 
- The following block of two lines is an operation table for boolean `Not`
- The next block of two lines are 'flattening' rules, that 'unpack' nested values
- The last four blocks are the first four operation tables defined for nested values

## Why Two Operation Tables for One Operation?
We include both operation tables because we can't rewrite into naked names, they must be wrapped in at least one `term`. This means that the output of an operation rewrite is always going to be a term. So, really, the first set of operation tables can be thought of as providing a kind of syntactic sugar.

## Extending the Example
- One can take `rules.urn` and apply it with a different `terms` file
- One can add other boolean operation tables
- One can add [variable aliases](./aliases.md) for formulaic calculation