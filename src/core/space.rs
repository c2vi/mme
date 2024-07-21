use enum_dispatch::enum_dispatch;

use crate::implementors::slint_widget::SlintWidget;
use crate::error::MmeResult;

// this is the main type of this project
// a Widget represents any kind of "screen realestate"
// be it a Webview, slint, Xwindow, ...
pub struct Space {
    implementation: SpaceImplementor
}

// an enum over all of what types such "screen realestate" could have
#[enum_dispatch]
pub enum SpaceImplementor {
    //Xwindow {},
    //WaylandWindow {},
    SlintWidget,
    //QtWidget {},
    //GtkWidget {},
    //Html {},
    //QuarzWindow {},
    //NtWindow {},
    //Activity {},
}

// common behaviour for all Rendertypes
#[enum_dispatch(WidgetImplementor)]
pub trait SpaceTrait {
    fn put_top(self, pos: Position, widget: Space) -> MmeResult<()>;

    fn put_top_full(self, widget: Space) -> MmeResult<()>;
}

pub struct Position {
    x: u32,
    y: u32,
    z: u32,
}


