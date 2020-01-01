mod clipboard;

mod paintable;
mod selection;

pub use clipboard::{get_image_from_clipboard, put_image_to_clipboard, ClipboardError};
pub use paintable::Paintable;
pub use selection::Selection;
