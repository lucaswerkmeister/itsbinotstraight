# it’s bi not straight

[A Twitter bot](https://twitter.com/ItsBiNotHetero)
posting riffs on the “it’s the bible not the straightble” joke.

## Development

The bot currently exists in two versions,
which are mostly feature-identical
(there are minor differences in Unicode handling).
The JS version came first, the Rust version is the one currently deployed.

## Rust

Run `cargo run` to generate a tweet text (without sending a tweet),
or `cargo test` to run unit tests.
Cargo will automatically download and build dependencies as needed.

`src/lib.rs` is the meat of the bot, generating the tweet texts.
`src/main.rs`, the main binary (and default for `cargo run`), simply generates one tweet text and prints it out.
`src/bin/tweet.rs` is the binary that actually sends a tweet,
loading credentials from the `.env` file or process environment,
and the one that is deployed to the server.
You can run it directly with `cargo run --bin tweet`.

## JS

Run `npm install` to install dependencies.

`tweet.js` is the meat of the bot, generating the tweet texts.
If loaded as the main module (i. e. `node tweet.js`),
it creates one text sample and prints it to standard output.
(You can also specify a number of lines to generate, e. g. `node tweet.js 25`.)

`index.js` loads credentials from the `.env` file or process environment,
and sends a single tweet.

## Deployment

For both versions, you build an image suitable as a [portable service](https://systemd.io/PORTABLE_SERVICES.html) –
using `./make-image` for the Rust version, or `mkosi` for the JS version.
An `.env` file with valid credentials must exist at image build time
and will be included in the image.
(The credentials are sensitive,
so they are not included in this repository
and you should not distribute the image containing them.
See `.env.template` for the format.)

Copy the resulting image (`itsbinotstraight/`) on some server into `/var/lib/portables/`,
attach it e. g. with `portablectl attach -p trusted itsbinotstraight`
and enable it with `systemctl enable --now itsbinotstraight.timer`.
(You can also send a single tweet with `systemctl start itsbinotstraight.service` first, to see if it works.)

For the JS version, the image must be attached with some profile that allows w+x memory,
i. e. one with `MemoryDenyWriteExecute=no`,
which is not the case with the default profile.
The only standard profile satisfying this condition is the “trusted” one;
however, you can create your own copy of the “default” profile with `MemoryDenyWriteExecute=no`
by placing an appropriately edited version of `/usr/lib/systemd/portable/profile/default/` below `/etc/systemd/portable/profile/`.

See also `DEPLOYING.md`.

## License

The code in this repository is released under the AGPL v3,
as provided in the `LICENSE` file.

