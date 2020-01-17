use crate::commands;
use crate::AppState;
use druid::{KeyCode, MenuDesc, MenuItem, RawMods};

pub(crate) fn make_menu(app: &AppState) -> MenuDesc<AppState> {
    MenuDesc::empty().append(file_menu(app)).append(edit_menu(app)).append(about_menu(app))
}

fn file_menu(app: &AppState) -> MenuDesc<AppState> {
    MenuDesc::new(L!("menu-file-menu"))
        .append(new())
        .append(new_from_clipboard())
        .append_separator()
        .append(open())
        .append_separator()
        .append(save().disabled_if(|| app.editor.canvas.is_none()))
        .append_separator()
        .append(exit())
}

fn edit_menu(_app: &AppState) -> MenuDesc<AppState> {
    MenuDesc::new(L!("menu-edit-menu"))
        .append(undo())
        .append(redo())
        .append_separator()
        .append(copy())
        .append(paste())
}

fn about_menu(_app: &AppState) -> MenuDesc<AppState> {
    MenuDesc::new(L!("menu-about-menu")).append(about())
}

macro_rules! register_menu_items {
    ($($name:ident => ($sel:literal, $cmd:expr $(, $mods:ident, $keycode:ident)? )),*) => {
        $(
        fn $name() -> MenuItem<AppState> {
            MenuItem::new(L!($sel), $cmd)
                $( .hotkey(RawMods::$mods, KeyCode::$keycode) )?
        })*
    }
}

register_menu_items! {
    // files
    open => ("menu-file-open", commands::file_open_command(), Ctrl, KeyO),
    new => ("menu-file-new", commands::FILE_NEW_ACTION, Ctrl, KeyN),
    new_from_clipboard => ("menu-file-new-clipboard", commands::FILE_NEW_CLIPBOARD_ACTION),
    save => ("menu-file-save-as", commands::file_save_as_command(), CtrlShift, KeyS),
    exit => ("menu-file-exit", commands::FILE_EXIT_ACTION, Alt, F4),

    // edit
    undo => ("menu-edit-undo", commands::EDIT_UNDO_ACTION, Ctrl, KeyZ),
    redo => ("menu-edit-redo", commands::EDIT_REDO_ACTION, CtrlShift, KeyZ),
    copy => ("menu-edit-copy", commands::EDIT_COPY_ACTION, Ctrl, KeyC),
    paste => ("menu-edit-paste", commands::EDIT_PASTE_ACTION, Ctrl, KeyV),

    // about
    about => ("menu-about-test", commands::ABOUT_TEST_ACTION)
}
