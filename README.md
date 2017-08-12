# ll: Long Lister

This is a developer-oriented replacement for `ls -lah`.

It uses unassigned Unicode code points to differentiate between different file
types. And if you so happen to use [Nerd Fonts](https://github.com/ryanoasis/nerd-fonts),
the result is awesome. In all other cases, it's likely just ugly.

With iTerm2 (macOS) and the Nerd Font called "Knack Regular Nerd Font Complete",
`ll`'s output looks like this:

![Screenshot](screenshot.png)

## To install:

Prerequisites:
- Go
- [Glide](http://glide.sh)
- And probably Nerd Fonts

```console
$ brew install go glide
```

You can install particular Nerd Fonts fonts [with these instructions](https://github.com/ryanoasis/nerd-fonts#option-4-homebrew-fonts)

Grab this repo and build:

```console
$ git clone $THIS_REPO
$ glide install
$ go install ll.go
```

If Go complains about $GOBIN not being set:

```console
$ GOBIN=/usr/local/bin install ll.go
```

## Troubleshooting

- You definitely need to install nerd-fonts and configure your terminal to use
  one of the Nerd Fonts fonts, or else this will be really ugly.
- Some shells alias `ll` to `ls -lah`. You may need to add `alias ll=$GOBIN/ll`
  to your .bashrc, .xshrc, .profile, etc.
- You may need to add `$GOBIN` to your `$PATH` or put `ll` in `/usr/local/bin`
