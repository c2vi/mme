use enum_dispatch::enum_dispatch;

use crate::implementors::slint_widget::SlintWidget;
use crate::error::MmeResult;

// this is the main type of this project
// a Widget represents any kind of "screen realestate"
// be it a Webview, slint, Xwindow, ...
pub struct Widget {
    implementation: WidgetImplementor
}

// an enum over all of what types such "screen realestate" could have
#[enum_dispatch]
pub enum WidgetImplementor {
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
pub trait WidgetTrait {
    fn put_top(self, pos: Position, widget: Widget) -> MmeResult<()>;

    fn put_top_full(self, widget: Widget) -> MmeResult<()>;
}

pub struct Position {
    x: u32,
    y: u32,
    z: u32,
}


