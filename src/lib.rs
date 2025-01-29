mod animated;
#[cfg(feature = "test-api")]
mod public_api_test;
pub use animated::Animated;
pub use animated::Easing;
mod traits;
pub use traits::AnimationTime;
pub use traits::FloatRepresentable;
pub use traits::Interpolable;
pub use traits::TestStruct;
