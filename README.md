# geo-traits

Exploring geo traits for Rust

## Rationale

Instead of having common structs for standardized _storage_ of geospatial types, define common traits for standardized _access_ of geospatial data in whatever format. The potential of this is that algorithms could be written once and then used on many different sources of geo data with zero copy.

Possible trait implementations:

- Existing `geo-types` structs
- GeoArrow
- FlatGeobuf
- GEOS

See also [this `geo` discussion](https://github.com/georust/geo/discussions/838).

## Design decisions


## Prior work

- [Existing traits in `rust-postgis`](https://github.com/andelf/rust-postgis/blob/master/src/types.rs).


## Acknowledgements

The existing traits in `rust-postgis` were used as a starting point, under the MIT license.
