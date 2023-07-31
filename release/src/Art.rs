//! # Art
//! //! A library for modeling artistic concepts.

// 使用pub use 来导出模块在引入时候可以省略中间的包名
pub use self::kinds::PrimaryColor; 
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
// --snip--
 /// The primary colors according to the RYB color model.
 pub enum PrimaryColor {
 Red,
 Yellow,
 Blue,
 }
  /// The secondary colors according to the RYB color model.
 pub enum SecondaryColor {
 Orange,
 Green,
 Purple,
 }
}
pub mod utils {
// --snip--
 use super::kinds::*;

 /// Combines two primary colors in equal amounts to create
 /// a secondary color.
 pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
 SecondaryColor::Orange
 }
}