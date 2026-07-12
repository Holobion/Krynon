//! The views module contains the components for all Layouts and Routes for our app.

mod home;
pub use home::Home;

mod compare;
pub use compare::Compare;

mod category_page;
pub use category_page::CategoryPage;

mod workspace_page;
pub use workspace_page::WorkspacePage;

mod about;
pub use about::About;

mod navbar;
pub use navbar::Navbar;
