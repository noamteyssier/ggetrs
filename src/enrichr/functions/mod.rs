mod add_background;
mod add_list;
mod enrich;
mod get_libraries;
mod shorthand;
mod view_list;
pub use add_background::add_background;
pub use add_list::add_list;
pub use enrich::enrich;
pub use get_libraries::get_libraries;
pub use shorthand::shorthand;
pub use view_list::view_list;

pub const ENRICHR_URL: &str = "https://maayanlab.cloud/Enrichr";
pub const SPEEDRICHR_URL: &str = "https://maayanlab.cloud/speedrichr";
