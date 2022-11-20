# it’s bi not straight

A [Mastodon](https://botsin.space/@ItsBiNotStraight) and [Twitter bot](https://twitter.com/ItsBiNotHetero)
posting riffs on the “it’s the bible not the straightble” joke.

## Development

The bot is written in Rust.
(An earlier version, which you can find in the Git history, was written in JS;
it was mostly feature-identical, with minor differences in Unicode handling.)

Run `cargo run` to generate a post text (without sending a post),
or `cargo test` to run unit tests.
Cargo will automatically download and build dependencies as needed.

`src/lib.rs` is the meat of the bot, generating the post texts.
`src/main.rs`, the main binary (and default for `cargo run`), simply generates one post text and prints it out.
`src/bin/post.rs` is the binary that actually sends a post,
loading credentials from the `.env` file or process environment,
and the one that is deployed to the server.
You can run it directly with `cargo run --bin post`.

## Deployment

Build an image suitable as a [portable service](https://systemd.io/PORTABLE_SERVICES.html), using `./make-image`.
An `.env` file with valid credentials must exist at image build time
and will be included in the image.
(The credentials are sensitive,
so they are not included in this repository
and you should not distribute the image containing them.
See `.env.template` for the format.)

Copy the resulting image (`itsbinotstraight/`) on some server into `/var/lib/portables/`,
attach it e. g. with `portablectl attach -p trusted itsbinotstraight`
and enable it with `systemctl enable --now itsbinotstraight.timer`.
(You can also send a single post with `systemctl start itsbinotstraight.service` first, to see if it works.)

See also `DEPLOYING.md`.

## License

The code in this repository is released under the AGPL v3,
as provided in the `LICENSE` file.

