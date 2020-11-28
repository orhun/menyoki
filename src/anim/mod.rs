pub mod decoder;
pub mod settings;

use crate::image::Image;

/* Images to encode and FPS value */
pub type Frames = (Vec<Image>, u32);
