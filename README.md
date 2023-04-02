# CommandScript

Bash sucks but most other languages do not have good syntax for dealing with commands. CommandScript is a WIP experimental scripting language written in Rust, which aims to be a replacement for bash script. 

## Status

 - [x] Lexer
 - [ ] Parser
 - [ ] Interpreter

## Example Code

The following code is a simple example of a CLI which presents the user with a list of git branches and asks the to select one to checkout.

```
// Usage: cs checkout.cs

branches = $ git branch --format '%(refname:short)'

assert(
  branches.stdout !== "",
  "No branches found. Are you in a git repository?"
)

branches = split(branches.stdout, "\n")

idx = 0
while idx < length(branches) {
  $ echo $idx : $(nth(branches, idx))
  idx += 1
}

$ echo Enter the number for a branch to checkout

choice = false
while choice == false {
  input = read_line()
  if typeof(parsed_input = parse_number(input)) == 'Error' {
    parse_error = parsed_input
    $ echo Error parsing input ($input) as a number:
    $ echo   $(parse_error.message)
    continue
  }
  if (
    parsed_input < 0 ||
    parsed_input >= length(branches)
  ) {
    $ echo Parsed input must be between 0 and $(length(branches) - 1)
  }
  choice = math.floor(parsed_input)
}

$ git checkout $(nth(branches, choice))
```
