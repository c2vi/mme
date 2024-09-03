
#![ allow( warnings ) ]

mod core {
    pub mod mme;
    pub mod error;
    pub mod slot;
    pub mod presenter;
    pub mod layout;
    pub mod adapter;
}

pub use core::slot;
pub use core::presenter;
pub use core::layout;
pub use core::adapter;
pub use core::mme;
pub use core::error;


pub mod implementors {
    pub mod html;

    #[cfg(features = "qt")]
    pub mod qt_widget;

    #[cfg(features = "slint")]
    pub mod slint_widget;

    #[cfg(features = "x11")]
    pub mod x_window;
}
