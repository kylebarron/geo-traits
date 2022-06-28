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

## Design decisions/Discussion Points

### Require Random Access?

I don't see anything wrong with requiring random access and I figure it would make implementing algorithms easier.

### Geo Factory traits

For an algorithm to return the same concrete structure that was passed in, we need a concept of factory traits.

Ref

- https://github.com/georust/geo/issues/67#issuecomment-257710243


### Allow Implementing a subset of traits?

Currently `geo-types` implements [quite a few types](https://docs.rs/geo-types/latest/geo_types/#geometries), some of which are less popular, like `Rect` and `Triangle`. Presumably if `geo` is ever going to adopt a trait-based model for its algorithms, the traits will require a full mapping from the current selection of structs.

Should it be a requirement that any implementor implement _all_ traits? I think not. For one, some source formats don't map neatly onto all traits.

This would also allow a mix and match of `geo`'s structs and another zero-copy format. For example, if `geo`'s structs implemented geo factory traits, then a `geo` `Line` struct could be constructed seamlessly from three points in GeoArrow memory. An algorithm that takes in a `LineString` and a `Line` and returns a `LineString` could then be done with mostly GeoArrow memory, only using the `geo` struct for the intermediate `Line` representation.

## Prior work

- [Original `geo-types` issue from 2016](https://github.com/georust/geo/issues/67).
- [Existing traits in `rust-postgis`](https://github.com/andelf/rust-postgis/blob/master/src/types.rs).


## Acknowledgements

The existing traits in `rust-postgis` were used as a starting point, under the MIT license.
