//! geo traits implementation

#![allow(clippy::type_complexity)]

use geo_types::CoordNum;

// TODO: make this generic over CoordNum/CoordFloat
pub trait Point<T>
where
    T: CoordNum + Send + Sync,
{
    /// x component of this point
    fn x(&self) -> T;

    /// y component of this point
    fn y(&self) -> T;

    fn from_xy(x: T, y: T) -> Self;
}

pub trait LineString<'a, T>
where
    T: CoordNum + Send + Sync,
{
    type ItemType: 'a + Point<T>;
    type Iter: Iterator<Item = &'a Self::ItemType>;

    /// An iterator over the points in this LineString
    fn points(&'a self) -> Self::Iter;

    /// The number of points in this LineString
    fn num_points(&'a self) -> usize;

    /// Access to a specified point in this LineString
    /// Will return None if the provided index is out of bounds
    fn point(&'a self, i: usize) -> Option<Self::ItemType>;
}

pub trait Polygon<'a, T>
where
    T: CoordNum + Send + Sync,
{
    type ItemType: 'a + LineString<'a, T>;
    type Iter: Iterator<Item = &'a Self::ItemType>;

    // TODO: should rings be split into outer_ring and inner_rings?
    // https://github.com/georust/geo/pull/85#discussion_r96141112

    /// An iterator over the rings in this Polygon
    fn rings(&'a self) -> Self::Iter;

    /// The number of rings in this Polygon
    fn num_rings(&'a self) -> usize;

    /// Access to a specified point in this Polygon
    /// Will return None if the provided index is out of bounds
    fn ring(&'a self, i: usize) -> Option<Self::ItemType>;
}

pub trait MultiPoint<'a, T>
where
    T: CoordNum + Send + Sync,
{
    type ItemType: 'a + Point<T>;
    type Iter: Iterator<Item = &'a Self::ItemType>;

    /// An iterator over the points in this MultiPoint
    fn points(&'a self) -> Self::Iter;

    /// The number of rings in this MultiPoint
    fn num_points(&'a self) -> usize;

    /// Access to a specified point in this MultiPoint
    /// Will return None if the provided index is out of bounds
    fn point(&'a self, i: usize) -> Option<Self::ItemType>;
}

pub trait MultiLineString<'a, T>
where
    T: CoordNum + Send + Sync,
{
    type ItemType: 'a + LineString<'a, T>;
    type Iter: Iterator<Item = &'a Self::ItemType>;

    /// An iterator over the LineStrings in this MultiLineString
    fn lines(&'a self) -> Self::Iter;

    /// The number of lines in this MultiLineString
    fn num_lines(&'a self) -> usize;

    /// Access to a specified line in this MultiLineString
    /// Will return None if the provided index is out of bounds
    fn line(&'a self, i: usize) -> Option<Self::ItemType>;
}

pub trait MultiPolygon<'a, T>
where
    T: CoordNum + Send + Sync,
{
    type ItemType: 'a + Polygon<'a, T>;
    type Iter: Iterator<Item = &'a Self::ItemType>;

    /// An iterator over the Polygons in this MultiPolygon
    fn polygons(&'a self) -> Self::Iter;

    /// The number of polygons in this MultiPolygon
    fn num_polygons(&'a self) -> usize;

    /// Access to a specified polygon in this MultiPolygon
    /// Will return None if the provided index is out of bounds
    fn polygon(&'a self, i: usize) -> Option<Self::ItemType>;
}

pub trait Geometry<'a, T>
where
    T: CoordNum + Send + Sync,
{
    type Point: 'a + Point<T>;
    type LineString: 'a + LineString<'a, T>;
    type Polygon: 'a + Polygon<'a, T>;
    type MultiPoint: 'a + MultiPoint<'a, T>;
    type MultiLineString: 'a + MultiLineString<'a, T>;
    type MultiPolygon: 'a + MultiPolygon<'a, T>;
    type GeometryCollection: 'a + GeometryCollection<'a>;
    fn as_type(
        &'a self,
    ) -> GeometryType<
        'a,
        T,
        Self::Point,
        Self::LineString,
        Self::Polygon,
        Self::MultiPoint,
        Self::MultiLineString,
        Self::MultiPolygon,
        Self::GeometryCollection,
    >;
}

pub enum GeometryType<'a, T, P, L, Y, MP, ML, MY, GC>
where
    T: CoordNum + Send + Sync,
    P: 'a + Point<T>,
    L: 'a + LineString<'a, T>,
    Y: 'a + Polygon<'a, T>,
    MP: 'a + MultiPoint<'a, T>,
    ML: 'a + MultiLineString<'a, T>,
    MY: 'a + MultiPolygon<'a, T>,
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

// pub trait PointFactory {
//     // TODO: how to
//     fn from_xy(x: f64, y: f64) -> Box<dyn Point<f64>>;
// }
