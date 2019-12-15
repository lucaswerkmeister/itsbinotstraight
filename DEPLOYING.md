# Deploying it’s bi not straight

## TL;DR

```sh
sudo mkosi -f &&
sudo systemd-run --pty -p User="$USER" -p AmbientCapabilities=CAP_DAC_READ_SEARCH -p WorkingDirectory="$PWD" -E SSH_AUTH_SOCK="$SSH_AUTH_SOCK" casync make --without=user-names --store=luthien:/var/lib/casync/store/ luthien:/var/lib/portables/itsbinotstraight.caidx itsbinotstraight/ &&
ssh luthien sudo systemctl disable --now itsbinotstraight.timer &&
ssh luthien sudo portablectl detach itsbinotstraight &&
ssh luthien sudo casync extract --store=/var/lib/casync/store/ /var/lib/portables/itsbinotstraight.caidx /var/lib/portables/itsbinotstraight/ &&
ssh luthien sudo portablectl attach --profile default-with-JIT itsbinotstraight &&
ssh luthien sudo systemctl enable --now itsbinotstraight.timer
```

I haven’t tested this as a single monolithic command yet, though.

## Details

Build the image locally:

```sh
sudo mkosi -f
```

An incremental build (`-i`) would potentially speed this up,
but doesn’t work on my system,
for reasons I can’t be bothered to investigate.

Sync it to the server (see [this blog post](https://lucaswerkmeister.de/posts/2019/01/11/system-naming-scheme/) for background on the name):

```sh
sudo systemd-run --pty -p User="$USER" -p AmbientCapabilities=CAP_DAC_READ_SEARCH -p WorkingDirectory="$PWD" -E SSH_AUTH_SOCK="$SSH_AUTH_SOCK" casync make --without=user-names --store=luthien:/var/lib/casync/store/ luthien:/var/lib/portables/itsbinotstraight.caidx itsbinotstraight/
```

`casync` needs to be able to read the entire OS tree (hence `CAP_DAC_READ_SEARCH`),
but also use my user’s SSH config, that’s why we don’t just use plain `sudo`.
`--without=user-names` is necessary because the `systemd-journal-remote` group,
which owns `/var/log/journal/remote` in the Arch system,
does not exist on the Debian server.
(Specifying this option at extract time is not enough to prevent all issues.)

On the server, temporarily detach the image:

```sh
sudo systemctl disable --now itsbinotstraight.timer && sudo portablectl detach itsbinotstraight
```

`portablectl` will refuse to detach the image if the timer is enabled.

Extract the image:

```sh
sudo casync extract --store=/var/lib/casync/store/ /var/lib/portables/itsbinotstraight.caidx /var/lib/portables/itsbinotstraight/
```

If the target directory already exists,
`casync` should delete files not mentioned in the image on its own,
but I haven’t tried that out yet.

Attach the image again:

```sh
sudo portablectl attach --profile default-with-JIT itsbinotstraight && sudo systemctl enable --now itsbinotstraight.timer
```

Optionally check `systemctl list-timers` to see if we missed a post due to this deployment;
if yes, run `systemctl start itsbinotstraight` to manually trigger a post.
