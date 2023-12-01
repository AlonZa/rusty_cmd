# rusty_cmd

## Description
A simple Rust crate to be utilized to create new line-orirnted command interpreters

## Features

### Command Loop
Run a loop that each iteration will parse a command-line, and than run the command using its CommandHandler (A must have trait), and will send the handler the right arguments.

### Custom Command Prompt
Ability to customize the command prompt to fit the interpreter needs.

## Limitations
* When adding new commands, it is mandatory to use `CmdLoop.printer.print_wrapped()`

## Future Work
- [ ] Add Scrolling option
- [ ] Add keyboard interrupt handler
- [x] Add default `quit` and `help` commands
- [ ] Add `pre-loop` and `post-loop` functions as a trait, with default implementation
- [ ] Add colored prompt option
- [ ] Add history access using the arrow keys
- [ ] Add Tab completion