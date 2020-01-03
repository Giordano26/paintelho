use druid::{Data, PaintCtx, Rect, Size};
use image::DynamicImage;

use crate::edit::Edit;
use crate::plane::Planes;
use crate::{Paintable, Selection};

use std::sync::Arc;

// FIXME: Change name to Layer
#[derive(Data, Clone)]
pub struct CanvasData {
    selection: Option<Selection>,
    planes: Planes,
}

impl CanvasData {
    pub fn new(img: Arc<DynamicImage>) -> CanvasData {
        let mut planes = Planes::new();
        planes.push(img.clone());
        CanvasData { selection: None, planes }
    }

    pub fn save(&self, path: &std::path::Path) -> Result<Arc<DynamicImage>, std::io::Error> {
        let img = self.image();
        img.save(path)?;
        Ok(img)
    }

    pub fn selection(&self) -> Option<&Selection> {
        self.selection.as_ref()
    }

    pub fn image(&self) -> Arc<DynamicImage> {
        self.planes.merged().expect("There is at least plane in Canvas")
    }

    pub fn select_rect(&mut self, rect: Rect) {
        if rect.size() == Size::ZERO {
            self.selection = None;
        } else {
            self.selection = Some(Selection::rect(rect));
        }
    }
}

impl Paintable for CanvasData {
    fn paint(&self, paint_ctx: &mut PaintCtx) {
        self.planes.paint(paint_ctx);

        if let Some(selection) = self.selection.as_ref() {
            selection.paint(paint_ctx);
        }
    }

    fn paint_size(&self) -> Option<Size> {
        self.planes.paint_size()
    }
}

pub struct Paste {
    img: DynamicImage,
}

impl Paste {
    pub fn new(img: DynamicImage) -> Paste {
        Paste { img }
    }
}

#[must_use]
impl Edit<CanvasData> for Paste {
    fn apply(&self, data: &mut CanvasData) {
        data.planes.push(Arc::new(self.img.clone()));
    }

    fn description(&self) -> String {
        "Paste".to_string()
    }
}