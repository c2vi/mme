
#![ allow( warnings ) ]

mod core {
    pub mod mme;
    pub mod error;
    pub mod space;
    pub mod presenter;
    pub mod layout;
    pub mod adapter;
}

pub use core::space;
pub use core::presenter;
pub use core::layout;
pub use core::adapter;
pub use core::mme;
pub use core::error;


mod implementors {
    pub mod html;
    pub mod qt_widget;
    pub mod slint_widget;
    pub mod x_window;
}
