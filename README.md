# it’s bi not straight

[A Twitter bot](https://twitter.com/ItsBiNotHetero)
posting riffs on the “it’s the bible not the straightble” joke.

## Development

Run `npm install` to install dependencies.

`tweet.js` is the meat of the bot, generating the tweet texts.
If loaded as the main module (i. e. `node tweet.js`),
it creates one text sample and prints it to standard output.

`index.js` loads credentials from the `.env` file or process environment,
and sends a single tweet.

## Deployment

Run `mkosi` to build an image suitable as a [portable service](https://systemd.io/PORTABLE_SERVICES.html).
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

Note that the image must be attached with some profile that allows w+x memory,
i. e. one with `MemoryDenyWriteExecute=no`,
which is not the case with the default profile.
The only standard profile satisfying this condition is the “trusted” one;
however, you can easily create your own copy of the “default” profile with `MemoryDenyWriteExecute=no`
by placing an appropriately edited version of `/usr/lib/systemd/portable/profile/default/` below `/etc/systemd/portable/profile/`.

## License

The code in this repository is released under the AGPL v3,
as provided in the `LICENSE` file.

