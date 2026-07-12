//! The components module contains all shared components for our app.

mod hero;
#[allow(unused_imports)]
pub use hero::Hero;

mod weight_slider;
pub use weight_slider::WeightSlider;

mod product_card;
pub use product_card::ProductCard;
