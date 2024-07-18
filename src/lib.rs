
#![ allow( warnings ) ]

mod core {
    pub mod widget;
    pub mod mme;
    pub mod error;
}

pub use core::widget;
pub use core::mme;
pub use core::error;

mod implementors {
    pub mod html;
    pub mod qt_widget;
    pub mod slint_widget;
    pub mod x_window;
}
