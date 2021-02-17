# gdk4

[Project site](https://gtk-rs.org/)

Rust bindings of __GDK 4__, part of [gtk4-rs](https://github.com/gtk-rs/gtk4-rs/).

__Required Rust version__: 1.48+

## Documentation

- [Stable] TODO
- [Development](https://gtk-rs.org/gtk4-rs/gdk4/)
- [The C documentation](https://developer.gnome.org/gdk4/stable/)

## Building

__gdk4__ expects the development files of various libraries to be installed on your machine.You can follow [the installation instructions](https://www.gtk.org/docs/installations/)

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
gdk = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gdk4" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
gtk = {version = "0.1", package = "gtk4"}
gdk = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gdk4" }
```

### See Also

- [glib](https://crates.io/crates/glib)
- [gio](https://crates.io/crates/gio)
- [gsk4](https://crates.io/crates/gsk4)
- [gtk4](https://crates.io/crates/gtk4)

## License

The Rust bindings of __gsk4__ are available under the MIT License, please refer to it.
