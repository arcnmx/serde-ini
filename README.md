# serde-ini

[![travis-badge][]][travis] [![release-badge][]][cargo] [![docs-badge][]][docs] [![license-badge][]][license]

`serde_ini` provides a serde `Serializer` and `Deserializer` for the [INI format](https://en.wikipedia.org/wiki/INI_file).
The format is rather limited, only allowing top level keys to be maps or structs
and all values and keys must be in the form of a `String`. This implementation
will try to use `ToString` and `FromStr` where appropriate for numeric values.
Sequences, tuples, bytes, bools, and some other data types are not supported.

## [Documentation][docs]

See the [documentation][docs] for up to date API documentation.

[travis-badge]: https://img.shields.io/travis/arcnmx/serde-ini/master.svg?style=flat-square
[travis]: https://travis-ci.org/arcnmx/serde-ini
[release-badge]: https://img.shields.io/crates/v/serde_ini.svg?style=flat-square
[cargo]: https://crates.io/crates/serde_ini
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: http://arcnmx.github.io/serde-ini/serde_ini/
[license-badge]: https://img.shields.io/badge/license-MIT-ff69b4.svg?style=flat-square
[license]: https://github.com/arcnmx/serde-ini/blob/master/COPYING
