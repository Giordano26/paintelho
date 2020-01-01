use druid::widget::{Align, Either, Flex, Label, Padding, Scroll, WidgetExt};
use druid::{
    theme, AppDelegate, AppLauncher, Application, Color, Data, DelegateCtx, Env, Event, Lens,
    LensExt, LocalizedString, UnitPoint, Widget, WindowDesc, WindowId,
};
use paintr::{get_image_from_clipboard, put_image_to_clipboard, CanvasData};

macro_rules! L {
    ($str:literal) => {
        $crate::LocalizedString::new($str)
    };
}

mod commands;
mod menu;
mod widgets;
use std::sync::Arc;
use widgets::{
    notif_bar::{Notification, NotificationContainer},
    Canvas, Named,
};

fn main() {
    let app_state = AppState { notifications: Arc::new(Vec::new()), image: None };

    let main_window = WindowDesc::new(ui_builder)
        .title(L!("paint-app-name"))
        .menu(menu::make_menu(&app_state))
        .window_size((800.0, 600.0));

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .configure_env(|env| {
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::rgb8(0, 0x77, 0x88));
        })
        // .use_simple_logger()
        .launch(app_state)
        .expect("launch failed");
}

struct Delegate;

type Error = Box<dyn std::error::Error>;

#[derive(Clone, Data, Lens)]
struct AppState {
    notifications: Arc<Vec<Notification>>,
    image: Option<(Arc<std::path::PathBuf>, CanvasData)>,
}

impl AppState {
    fn show_notification(&mut self, n: Notification) {
        Arc::make_mut(&mut self.notifications).push(n);
    }

    fn do_open_image(&mut self, path: &std::path::Path) -> Result<(), Error> {
        self.image =
            Some((Arc::new(path.to_owned()), CanvasData::new(Arc::new(image::open(path)?))));
        Ok(())
    }

    fn do_new_image(&mut self) -> Result<(), Error> {
        let img = get_image_from_clipboard()?
            .ok_or_else(|| "Clipboard is empty / non-image".to_string())?;

        self.image = Some((
            Arc::new(std::path::Path::new("Untitled").into()),
            CanvasData::new(Arc::new(img)),
        ));
        Ok(())
    }

    fn do_save_as_image(&mut self, path: &std::path::Path) -> Result<(), Error> {
        let (_, canvas) = self.image.take().ok_or_else(|| "No image was found.")?;
        let canvas = CanvasData::new(canvas.save(path)?);
        self.image = Some((Arc::new(path.to_path_buf()), canvas));
        Ok(())
    }

    fn do_copy(&mut self) -> Result<bool, Error> {
        let img = self
            .canvas()
            .and_then(|canvas| canvas.selection().map(|sel| sel.image(canvas.image())));

        let img = match img {
            None => return Ok(false),
            Some(img) => img,
        };

        put_image_to_clipboard(&img)?;
        Ok(true)
    }

    fn image_file_name(&self) -> String {
        match &self.image {
            None => "Untitled".into(),
            Some((path, _)) => path.to_string_lossy().into(),
        }
    }

    fn update_menu(&self, ctx: &mut DelegateCtx) {
        ctx.submit_command(
            druid::Command::new(druid::commands::SET_MENU, menu::make_menu(self)),
            None,
        );
    }

    fn canvas(&self) -> Option<&CanvasData> {
        self.image.as_ref().map(|(_, canvas)| canvas)
    }

    fn status(&self) -> Option<String> {
        Some(self.canvas()?.selection()?.description())
    }
}

impl Delegate {
    fn handle_command(
        &mut self,
        data: &mut AppState,
        ctx: &mut DelegateCtx,
        cmd: &druid::Command,
    ) -> Result<(), Error> {
        match &cmd.selector {
            &commands::FILE_EXIT_ACTION => {
                ctx.submit_command(druid::commands::CLOSE_WINDOW.into(), None);
            }
            &commands::FILE_NEW_ACTION => {
                data.do_new_image()?;
                data.show_notification(Notification::info("New file created"));
                data.update_menu(ctx);
            }
            &druid::commands::OPEN_FILE => {
                let info = cmd
                    .get_object::<druid::FileInfo>()
                    .ok_or_else(|| "api violation".to_string())?;
                data.do_open_image(info.path())?;
                data.show_notification(Notification::info(format!(
                    "{} opened",
                    data.image_file_name()
                )));
                data.update_menu(ctx);
            }
            &druid::commands::SAVE_FILE => {
                let info = cmd
                    .get_object::<druid::FileInfo>()
                    .ok_or_else(|| "api violation".to_string())?;
                data.do_save_as_image(info.path())?;
                data.show_notification(Notification::info(format!(
                    "{} saved",
                    data.image_file_name()
                )));
                data.update_menu(ctx);
            }

            &commands::EDIT_COPY_ACTION => {
                if data.do_copy()? {
                    data.show_notification(Notification::info("Copied"));
                }
            }

            _ => (),
        }

        Ok(())
    }
}

impl AppDelegate<AppState> for Delegate {
    fn event(
        &mut self,
        event: Event,
        data: &mut AppState,
        _env: &Env,
        ctx: &mut DelegateCtx,
    ) -> Option<Event> {
        match event {
            Event::Command(ref cmd) => {
                if let Err(err) = self.handle_command(data, ctx, cmd) {
                    data.show_notification(Notification::error(err.to_string()));
                }
            }

            _ => (),
        };

        Some(event)
    }

    fn window_removed(
        &mut self,
        _id: WindowId,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        // FIXME: Use commands::QUIT_APP
        // It do not works right now, maybe a druid bug
        Application::quit();
    }
}

fn ui_builder() -> impl Widget<AppState> {
    let text = L!("paintr-front-page-welcome");
    let label = Label::new(text.clone());

    let image_lens = AppState::image.map(
        |it| it.clone().map(|it| it.1),
        |to: &mut _, from| {
            if let Some(s) = to.as_mut() {
                if let Some(f) = from {
                    s.1 = f;
                }
            }
        },
    );

    let main_content = Either::new(
        |data: &AppState, &_| !data.image.is_some(),
        Align::centered(Padding::new(10.0, label)),
        Align::centered(Padding::new(
            10.0,
            Named::new(Scroll::new(Canvas::new().lens(image_lens)), |data: &AppState, _env: &_| {
                data.image_file_name()
            }),
        )),
    );

    Flex::column()
        .with_child(NotificationContainer::new(main_content, AppState::notifications), 1.0)
        .with_child(
            Label::new(|data: &AppState, _env: &Env| data.status().unwrap_or_default())
                .align(UnitPoint::RIGHT)
                .padding((5.0, 3.0))
                .background(Color::rgb(0.5, 0.3, 0.5))
                .env_scope(|env| {
                    env.set(theme::TEXT_SIZE_NORMAL, 12.0);
                }),
            0.0,
        )
}
