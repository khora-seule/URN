# Alias Example in URN

For our Alias example we have a `terms.urn` that looks like this:
```
{ 
    {
        This is a { x } example!
    }
    {
        Or is it really a { y } example?
    }
}
```

And we have two `rules` files: `negative.urn` and `positive.urn`. The first looks like this:
```
{ x } { terrible }
{ y } { great }
```
The second looks like this:
```
{ y } { terrible }
{ x } { great }
```

## Reading the Rules
We can read the rules off of `rules.urn` line by line, since each line has exactly two terms. Each looks very similar to the other -- maybe you didn't spot the difference at first. If you still haven't: `x` and `y` have swapped position with each other between the two sets of `rules`. This has the expected effect on the output when running `terms.urn` using the opposite `rules` file.

## Extending the Example
- One can add interdependent aliases
- One can add operations that aliases abstract over the input to
- One can do a kind of 'macro expansion' by chaining aliases 