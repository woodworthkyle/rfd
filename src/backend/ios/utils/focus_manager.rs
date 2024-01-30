use objc_id::Id;

use super::{IUIApplication, IUIWindow, UIApplication, UIWindow};

pub struct FocusManager {
    key_window: Option<Id<UIWindow>>,
}

impl FocusManager {
    pub fn new() -> Self {
        let app = UIApplication::shared_application();
        let key_window = app.key_window();

        Self { key_window }
    }
}

impl Drop for FocusManager {
    fn drop(&mut self) {
        if let Some(win) = &self.key_window {
            win.make_key_and_order_front();
        }
    }
}
