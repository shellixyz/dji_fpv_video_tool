pub mod overlay;
pub mod dji;
pub mod tile_resize;
pub mod tile;
pub mod region;
pub mod coordinates;
pub mod item;

use hd_fpv_osd_font_tool::dimensions::Dimensions as GenericDimensions;

pub type Dimensions = GenericDimensions<u8>;

pub use region::Region as Region;
pub use coordinates::{
    Coordinate,
    Coordinates,
    SignedCoordinate,
    SignedCoordinates,
    SignedRange as CoordinatesRange};
