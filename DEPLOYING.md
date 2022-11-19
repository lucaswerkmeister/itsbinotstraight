# Deploying it’s bi not straight

## TL;DR

```sh
./deploy
```

## Details

Build the image locally.
The [requirements for portable images](https://systemd.io/PORTABLE_SERVICES/#requirements-on-images) are pretty simple,
so we can build a minimal image with a hand-written script:

```sh
sudo ./make-image
```

Sync it to the server (see [this blog post](https://lucaswerkmeister.de/posts/2019/01/11/system-naming-scheme/) for background on the name):

```sh
sudo systemd-run --pty -p User="$USER" -p AmbientCapabilities=CAP_DAC_READ_SEARCH -p WorkingDirectory="$PWD" -E SSH_AUTH_SOCK="$SSH_AUTH_SOCK" casync make --without=user-names --store=galadriel:/var/lib/casync/store/ galadriel:/var/lib/portables/itsbinotstraight.caidx itsbinotstraight/
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

Make it world-searchable (`g+x`).
Locally, we don’t want this, since there’s a world-readable `.env` inside the image;
after deployment, however, that file is instead protected by all of `/var/lib/portables/` not being world-searchable,
and the image itself must be world-searchable so that the unprivileged service can change directory into it
(after having had the image bind-mounted into its mount namespace so it’s not affected by the mode of `/var/lib/portables/`):

```sh
sudo chmod 755 /var/lib/portables/itsbinotstraight/
```

Attach the image again:

```sh
sudo portablectl attach itsbinotstraight && sudo systemctl enable --now itsbinotstraight.timer
```

Optionally check `systemctl list-timers` to see if we missed a post due to this deployment;
if yes, run `systemctl start itsbinotstraight` to manually trigger a post.
