ccntool
=======

This repository is intended to be used with [openDCIM](https://opendcim.org)
and can be used to query the database to get information about a certain
network wallsocket.

The user has to input a wallsocket description (hardcoded for our usecase) and
the query will return the switch name, switch ip, specific switch port and its
description.
It can be used as a quick glance on what configuration you can expect from a
wallsocket port as well as to prepare information to pass on to the second
level support.

The project is divided in a library, `ccntool_core` which provides commonly
shared functions between different frontends.
You get to choose between `ccntool_cli`, `ccntool_tui` and `ccntool_gui`.
The most polished version is `ccntool_gui`, others may or may not be expanded
in the future.

Because of its very specific use-case this code is not published to
[crates.io](crates.io).
If there is any actual demand for that please open a ticket.

## 🛍️ Installation

You need at least rust version 1.67 to compile this project installed.

1. `git clone https://github.com/hs-duesseldorf/ccntool.git`
2. `cd ccntool`
3. `cargo build --release`

## 🛠️ How to use

You need to pass an user, a password and a hostname to connect to your
database.
Using the gui you can define it over the settings button, CLI and TUI user
will need to create a file called `.env` alongside their binary which should
look like that:

```
DCIMHOST=dcim.my.tld
DCIMUSER=user
DCIMPASSWORD=password
```

If done correctly, all frontends should be self-explanatory and usage is
possible.
The gui does provide a list of all valid ports you can query when a connection
can be made.

## 🗺️ Roadmap

In the future, this tool aims to use REST calls to provide more information
about a port.

If possible, it would be nice to compile a working web application from
the gui.

## ⚠️ Caveats

As noted above, the regex used is very specific to our use case and will need
to be changed.

Your openDCIM installation will need to have switches deployed which are
connected to patchpanels that finally are connected to wallsockets.
The wallsocket description is part of the notes field of a patchpanel, which
is the identifier used as user input on all frontends.

## 🤝 Contribution

If you do happen to use this tool, feel free to participate.
Tickets and pullrequests are welcome.

## 📝 License

This project is licensed under [MIT license](https://github.com/hs-duesseldorf/ccntool/blob/main/LICENSE)
except for its assets.
The file `HSDSans-Regular.ttf` is owned by [HS Düsseldorf](https://hs-duesseldorf.de)
and permission was granted to use it for this project.
If you do want to use this font, please seek permission there.
