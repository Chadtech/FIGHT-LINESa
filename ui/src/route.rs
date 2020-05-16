use crate::route::Route::Title;
use seed::Url;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

pub enum Route {
    Title,
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

pub fn parse(url: Url) -> Option<Route> {
    if url.path().is_empty() {
        return Some(Title);
    }
    None
}
