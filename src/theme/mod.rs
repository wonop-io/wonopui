mod brand_guide;
mod context;
mod defaults;
mod types;

pub use brand_guide::BrandGuide;
pub use context::{ThemeProvider, BrandGuideContext};
pub use defaults::BRANDGUIDE;
pub use types::*;

#[cfg(feature = "ThemeProvider")]
pub use context::{use_brandguide, use_set_brandguide};