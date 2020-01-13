# soup [![Build Status](https://travis-ci.org/whizsid/soup-rs.png?branch=master)](https://travis-ci.org/whizsid/soup-rs) 

__Rust__ bindings and wrappers for __libsoup__.

<aside class="notice">
NOTE:- This is not the official libsoup Rust bindings repo. Official repository currently not supporting for latest gtk-rs stack.
</aside>

[GNOME Official Repo](https://gitlab.gnome.org/World/Rust/soup-rs/)

## Building

__soup-rs__ expects __GTK+__, __GLib__ and __libsoup__ files to be installed on your system.

## Using

```toml
[dependencies]
soup-rs = { git = "https://github.com/whizsid/soup-rs.git" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
gtk = "0.2"
soup-rs = { git = "https://github.com/whizsid/soup-rs.git" }
```

## Contribute

Contributor you're welcome!

See the general [bindings documentation](http://gtk-rs.org/docs/glib/).

Most of the bindings ([`src/auto`](src/auto)) are generated by [gir](https://github.com/gtk-rs/gir) using [this configuration file](Gir.toml). After editing `Gir.toml` the sources can be regenerated with

```shell
> make gir
```

When opening a PR please put the changes to the `src/auto` directory in a separate commit.

## License

__soup-rs__ is available under the MIT License, please refer to it.
