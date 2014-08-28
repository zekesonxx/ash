# ash
Ash is a pure-Rust shell. It doesn't do much useful things, yet.
I'm writing this to learn Rust, not to actually make a viable alternative to bash/zsh. Although if I do that would be pretty awesome.

## Running

    $ cargo build
    $ ./target/ash
or

    $ cargo run

## Completed
* `cd` command
* Prompt displays ~ for paths within home

## Current Goals/todo
* Proper process running, piping to/from stdin/out/err
* Meta environment variables, `PWD` and the like
* Get hostname for prompt
* Command history
* Code cleanup
* No lint errors

## Long-term goals
* PS1/PS2 prompt formatting
* Tab completion
* Piping output
* Shell script interpreter

## Contributing
Please, do! Your code needs to work on both *nix and Windows though.

## License
MIT license, see `LICENSE` for full text