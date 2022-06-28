//! geo traits implementation

#![allow(clippy::type_complexity)]

pub trait Point: Send + Sync {
    /// x component of this point
    fn x(&self) -> f64;

    /// y component of this point
    fn y(&self) -> f64;
}

pub trait LineString<'a>: Send + Sync {
    type ItemType: 'a + Point;
    type Iter: Iterator<Item = &'a Self::ItemType>;

    /// An iterator over the points in this LineString
    fn points(&'a self) -> Self::Iter;

    /// The number of points in this LineString
    fn num_points(&'a self) -> usize;

    /// Access to a specified point in this LineString
    /// Will return None if the provided index is out of bounds
    fn point(&'a self, i: usize) -> Option<Self::ItemType>;
}

pub trait Polygon<'a>: Send + Sync {
    type ItemType: 'a + LineString<'a>;
    type Iter: Iterator<Item = &'a Self::ItemType>;

    /// An iterator over the rings in this Polygon
    fn rings(&'a self) -> Self::Iter;

    /// The number of rings in this Polygon
    fn num_rings(&'a self) -> usize;

    /// Access to a specified point in this Polygon
    /// Will return None if the provided index is out of bounds
    fn ring(&'a self, i: usize) -> Option<Self::ItemType>;
}

pub trait MultiPoint<'a>: Send + Sync {
    type ItemType: 'a + Point;
    type Iter: Iterator<Item = &'a Self::ItemType>;

    /// An iterator over the points in this MultiPoint
    fn points(&'a self) -> Self::Iter;

    /// The number of rings in this MultiPoint
    fn num_points(&'a self) -> usize;

    /// Access to a specified point in this MultiPoint
    /// Will return None if the provided index is out of bounds
    fn point(&'a self, i: usize) -> Option<Self::ItemType>;
}

pub trait MultiLineString<'a>: Send + Sync {
    type ItemType: 'a + LineString<'a>;
    type Iter: Iterator<Item = &'a Self::ItemType>;

    /// An iterator over the LineStrings in this MultiLineString
    fn lines(&'a self) -> Self::Iter;

    /// The number of lines in this MultiLineString
    fn num_lines(&'a self) -> usize;

    /// Access to a specified line in this MultiLineString
    /// Will return None if the provided index is out of bounds
    fn line(&'a self, i: usize) -> Option<Self::ItemType>;
}

pub trait MultiPolygon<'a>: Send + Sync {
    type ItemType: 'a + Polygon<'a>;
    type Iter: Iterator<Item = &'a Self::ItemType>;

    /// An iterator over the Polygons in this MultiPolygon
    fn polygons(&'a self) -> Self::Iter;

    /// The number of polygons in this MultiPolygon
    fn num_polygons(&'a self) -> usize;

    /// Access to a specified polygon in this MultiPolygon
    /// Will return None if the provided index is out of bounds
    fn polygon(&'a self, i: usize) -> Option<Self::ItemType>;
}

pub trait Geometry<'a>: Send + Sync {
    type Point: 'a + Point;
    type LineString: 'a + LineString<'a>;
    type Polygon: 'a + Polygon<'a>;
    type MultiPoint: 'a + MultiPoint<'a>;
    type MultiLineString: 'a + MultiLineString<'a>;
    type MultiPolygon: 'a + MultiPolygon<'a>;
    type GeometryCollection: 'a + GeometryCollection<'a>;
    fn as_type(
        &'a self,
    ) -> GeometryType<
        'a,
        Self::Point,
        Self::LineString,
        Self::Polygon,
        Self::MultiPoint,
        Self::MultiLineString,
        Self::MultiPolygon,
        Self::GeometryCollection,
    >;
}

pub enum GeometryType<'a, P, L, Y, MP, ML, MY, GC>
where
    P: 'a + Point,
    L: 'a + LineString<'a>,
    Y: 'a + Polygon<'a>,
    MP: 'a + MultiPoint<'a>,
    ML: 'a + MultiLineString<'a>,
    MY: 'a + MultiPolygon<'a>,
    GC: 'a + GeometryCollection<'a>,
{
    Point(&'a P),
    LineString(&'a L),
    Polygon(&'a Y),
    MultiPoint(&'a MP),
    MultiLineString(&'a ML),
    MultiPolygon(&'a MY),
    GeometryCollection(&'a GC),
}

pub trait GeometryCollection<'a> {
    type ItemType: 'a;
    type Iter: Iterator<Item = &'a Self::ItemType>;
    fn geometries(&'a self) -> Self::Iter;
}
