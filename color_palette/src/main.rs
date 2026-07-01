// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::QApp;
use qtbridge::include_bytes_qml;

mod basic_login;
mod paginated_source;
mod rest_service;
mod utils;

use basic_login::BasicLogin;
use paginated_source::PaginatedResource;
use rest_service::RestService;

fn main() {
    // Qt QML modules live under the `:/qt/qml/<Module>/` resource prefix, and the
    // QML hard-codes icon paths like `qrc:/qt/qml/ColorPalette/icons/qt.png`, so
    // everything is registered under `qt/qml/...` to match.
    include_bytes_qml!("icons/close.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/close_dark.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/delete.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/delete_dark.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/dots.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/dots_dark.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/edit.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/edit_dark.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/login.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/login_dark.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/logout.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/logout_dark.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/ok.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/ok_dark.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/plus.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/plus_dark.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/qt.png", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/testserver.png", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/update.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/update_dark.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/user.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/userMask.svg", "qt/qml/ColorPalette");
    include_bytes_qml!("icons/user_dark.svg", "qt/qml/ColorPalette");

    include_bytes_qml!("ColorPalette/ColorDialogDelete.qml", "qt/qml");
    include_bytes_qml!("ColorPalette/ColorDialogEditor.qml", "qt/qml");
    include_bytes_qml!("ColorPalette/ColorView.qml", "qt/qml");
    include_bytes_qml!("ColorPalette/Main.qml", "qt/qml");
    include_bytes_qml!("ColorPalette/qmldir", "qt/qml");
    include_bytes_qml!("ColorPalette/ServerSelection.qml", "qt/qml");
    include_bytes_qml!("ColorPalette/UserMenu.qml", "qt/qml");

    include_bytes_qml!("QtExampleStyle/Button.qml", "qt/qml");
    include_bytes_qml!("QtExampleStyle/Label.qml", "qt/qml");
    include_bytes_qml!("QtExampleStyle/Popup.qml", "qt/qml");
    include_bytes_qml!("QtExampleStyle/qmldir", "qt/qml");
    include_bytes_qml!("QtExampleStyle/TextField.qml", "qt/qml");
    include_bytes_qml!("QtExampleStyle/ToolBar.qml", "qt/qml");
    include_bytes_qml!("QtExampleStyle/ToolButton.qml", "qt/qml");
    include_bytes_qml!("QtExampleStyle/UIStyle.qml", "qt/qml");

    QApp::new()
        // Register the Rust back-end types into the "ColorPalette" QML module so
        // the .qml files can use RestService / PaginatedResource / BasicLogin
        // without an explicit import (they are part of the same module).
        .register::<RestService>()
        .register::<PaginatedResource>()
        .register::<BasicLogin>()
        .add_import_path("qrc:/qt/qml")
        .load_qml_from_file("qrc:/qt/qml/ColorPalette/Main.qml")
        .run();
}
