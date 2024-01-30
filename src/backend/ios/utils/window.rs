use objc::{msg_send, sel, sel_impl};
use objc_id::Id;

use super::nil;
use objc_foundation::{object_struct, INSObject};

use raw_window_handle::RawWindowHandle;

pub trait IUIWindow: INSObject {
    fn from_raw_window_handle(h: &RawWindowHandle) -> Id<Self> {
        match h {
            RawWindowHandle::UiKit(h) => {
                let id = h.ui_view as *mut Self;
                unsafe { Id::from_ptr(id) }
            }
            _ => unreachable!("unsupported window handle, expected: iOS"),
        }
    }

    fn make_key_and_order_front(&self) {
        let _: () = unsafe { msg_send![self, makeKeyAndOrderFront: nil] };
    }
}

object_struct!(UIWindow);
impl IUIWindow for UIWindow {}
