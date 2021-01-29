# DevilBotRustExt
***
An alternate instance of DevilBot
designed to speed up bot execution
in the superior Rust.

As of right now, the bot runs on
a seperate instance, with a seperate
prefix to be used for rust-only commands
in order to create seamlessness between
the ['old'](https://github.com/DevilJamJar/DevilBot)
bot and what will become the new one.
Eventually the bot will become entirely
written in Rust.

## Running DevilBot
***
### Taking Code
If you wish to use the code inside another
project, feel free to take code from here as
long as you follow the terms of the
[license](https://github.com/DevilJamJar/DevilBotRustExt/blob/master/LICENSE).
That includes credit, but be warned that it
may be more beneficial for you to use the
[docs](https://docs.rs/serenity/) for learning
purposes.
### Using DevilBot
The preferred method of using this code is by
simply inviting DevilBot to your guild. While this
RustExt is not currently included in the public
DevilBot, all of the functionality is, and the bot
can be invited [here](https://discord.com/api/oauth2/authorize?client_id=720229743974285312&permissions=67464257&scope=bot).
Alternatively, if you wish to see how the RustExt
functions, check the guild that it *is* active in,
[here](https://discord.gg/RzFmPf7). The prefix for
the RustExt is `&rust `, and the regular Python one
is `ow!`.
### Cloning DevilBot
This is less preferred than using DevilBot, and discouraged
due to the split between the Python and Rust extensions.
However, if you really want to clone the bot, you'll
need a `src/config.rs` file with a `public fn token`,
returning your bot's token. Then `cargo build` / `cargo
run`. Use `git clone` to get the files locally.
