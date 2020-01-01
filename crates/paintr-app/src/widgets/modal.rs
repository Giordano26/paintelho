//! Toast Box
//!
//! A widget represent a message box

use druid::{
    lens::{self, LensExt},
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, Point, Rect, Size, UpdateCtx,
    Widget, WidgetPod,
};

pub struct ModalContainer<T: Data, M: Data, L: lens::Lens<T, M>> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    closure: Box<dyn Fn(&M, &Env) -> Option<Box<dyn Widget<M>>>>,
    lens: L,
    modal: Option<WidgetPod<M, Box<dyn Widget<M>>>>,
}

impl<T: Data, M: Data, L: lens::Lens<T, M>> ModalContainer<T, M, L> {
    pub fn new<W, F>(inner: impl Widget<T> + 'static, closure: F, lens: L) -> Self
    where
        W: Widget<M> + 'static,
        F: Fn(&M, &Env) -> Option<W> + 'static,
    {
        ModalContainer {
            inner: WidgetPod::new(inner).boxed(),
            closure: Box::new(move |data, env| match closure(data, env) {
                None => None,
                Some(w) => Some(Box::new(w)),
            }),
            lens,
            modal: None,
        }
    }
}

impl<T: Data, M: Data + 'static, L: lens::Lens<T, M>> Widget<T> for ModalContainer<T, M, L> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        // if we have modal, block all other event
        if let Some(model) = &mut self.modal {
            self.lens.with_mut(data, |data| {
                model.event(ctx, event, data, env);
            });

            return;
        }

        self.inner.event(ctx, event, data, env);
    }
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: Option<&T>, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);

        let changed = match old_data {
            Some(old_data) => self
                .lens
                .with(old_data, |old_data| self.lens.with(data, |data| !old_data.same(data))),
            _ => true,
        };

        if changed {
            self.modal = (*self.closure)(&self.lens.get(data), env)
                .map(|inner| WidgetPod::new(inner).boxed());
            ctx.invalidate();
        }
    }
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));

        if let Some(modal) = &mut self.modal {
            self.lens.with(data, |data| {
                let size = modal.layout(ctx, bc, data, env);
                modal.set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));
            });
        }

        size
    }
    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint_with_offset(paint_ctx, data, env);

        if let Some(modal) = &mut self.modal {
            self.lens.with(data, |data| {
                modal.paint_with_offset(paint_ctx, data, env);
            });
        }
    }
}
