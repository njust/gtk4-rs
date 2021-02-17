# gtk4

[Project site](https://gtk-rs.org/)

Rust bindings of __GTK 4__, part of [gtk4-rs](https://github.com/gtk-rs/gtk4-rs/).

__Required Rust version__: 1.48+.

## Documentation

- [Stable] TODO
- [Development](https://gtk-rs.org/gtk4-rs/gtk4/)
- [The C documentation](https://developer.gnome.org/gtk4/stable/)

## Building

__gtk4__ expects the development files of various libraries to be installed on your machine.You can follow [the installation instructions](https://www.gtk.org/docs/installations/)

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
gtk = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gtk4" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
gdk = {version = "0.1", package = "gdk4"}
gtk = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gtk4" }
```

### See Also

- [glib](https://crates.io/crates/glib)
- [gio](https://crates.io/crates/gio)
- [gdk4](https://crates.io/crates/gdk4)
- [gsk4](https://crates.io/crates/gsk4)

## License

The Rust bindings of __gtk4__ are available under the MIT License, please refer to it.
