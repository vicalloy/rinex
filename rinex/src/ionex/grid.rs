use thiserror::Error;
use std::ops::Rem;
#[cfg(feature = "serde")]
use serde::Serialize;

/// Grid definition Error
#[derive(Error, Debug)]
pub enum Error {
    #[error("faulty grid definition: `start` and `end` must be multiples of each other")] 
    GridStartEndError,
    #[error("faulty grid definition: `start` and `end` must be multiples of `spacing`")] 
    GridSpacingError,
}

/// Grid linear space,
/// starting from `start` ranging to `end` (included) 
/// with given spacing, defined in km.
#[derive(Debug, Clone, Default)]
#[derive(PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridLinspace {
    /// Grid start value, in km
    pub start: f32,
    /// Grid end value, in km
    pub end: f32,
    /// Grid scaping / increment value, in km
    pub spacing: f32,
}

impl GridLinspace {
    /// Builds a new Linspace definition
    pub fn new(start: f32, end: f32, spacing: f32) -> Result<Self, Error> {
        let r = end.rem(start);
        if r == 0.0 {
            if end.rem(spacing) == 0.0 {
                Ok(Self {
                    start,
                    end,
                    spacing,
                })
            } else {
                Err(Error::GridSpacingError)
            }
        } else {
            Err(Error::GridStartEndError)
        }
    }
    // Returns total distance, in km, covered by
    // this Grid linear space
    pub fn total_distance(&self) -> f32 {
        (self.end - self.start) * self.spacing
    }
    /// Returns true if self is a single point space
    pub fn is_single_point(&self) -> bool {
        (self.end == self.start) && self.spacing == 0.0
    }
}

impl From<(f32,f32,f32)> for GridLinspace {
    fn from (tuple:(f32,f32,f32)) -> Self {
        Self {
            start: tuple.0,
            end: tuple.1,
            spacing: tuple.2,
        }
    }
}

/// Reference Grid,
/// defined in terms of Latitude, Longitude and Altitude.
/// If 2D-TEC maps, static altitude is defined, ie.: 
/// start = end altitude and spacing = 0.
#[derive(Debug, Clone, Default)]
#[derive(PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Grid {
    /// Latitude
    pub latitude: GridLinspace,
    /// Longitude
    pub longitude: GridLinspace,
    /// Altitude
    pub height: GridLinspace,
}

impl Grid {
    /// Returns true if self is defined for 3D TEC map
    pub fn is_3d_grid(&self) -> bool {
        !self.is_2d_grid()
    }
    /// Returns true if self is defined to 2D TEC maps,
    /// ie.: static altitude ref point with no altitude space
    /// definition.
    pub fn is_2d_grid(&self) -> bool {
        self.height.is_single_point()
    }
    /// Returns total projected 2D area covered [km²]
    pub fn total_area(&self) -> f32 {
        self.latitude.total_distance() * self.longitude.total_distance()
    }
}
