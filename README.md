# **M**a**th** Expression Pars**er**

It parses math expressions!  

## Using

Usage is as follows:
```
$ mether {subcommand} [expression]
````

Where the subcommands can be:
- `calc`, which calculates the given expression
- `graph`, which varies the x variable, and graphs the output of the given expression in the terminal
- `cli`, interactive prompt

## Expression grammar

Basic operations are: `+`, `-`, `*`, `/`.  
Functions and variables can have abitrarly long names, as long as they are only composed of letters.  
There is no implicit multiplication: `2x`, `3(2*4)` or `(1+2)(3+4)` are considered invalid.  
Non decimal number are supported: `0x` prefix allows hexadecimal, `0o` allows octal and `0b` allows binary.

## Dependencies

- `thiserror`
- `clap`
- `termion`
- `num-traits`
- `itertools`
