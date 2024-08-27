use enum_dispatch::enum_dispatch;

use crate::implementors::slint_widget::SlintWidget;
use crate::implementors::qt_widget::QtWidgetSlot;
use crate::error::MmeResult;
use crate::presenter::Presenter;

// common behaviour for all SlotTypes
#[enum_dispatch]
pub trait SlotTrait {
    fn load(&mut self, presenter: Presenter) -> MmeResult<()>;

    //pub fn load_html(pressenter: HtmlPresenter) -> MmeResult<()> {}
}

// this is the main type of this project
// a Widget represents any kind of "screen realestate"
// be it a Webview, slint widget, Xwindow, ...

// an enum over all of what types such "screen realestate" could have
#[enum_dispatch(SlotTrait)]
pub enum Slot {
    //Xwindow {},
    //WaylandWindow {},
    //SlintWidget,
    QtWidgetSlot,
    //GtkWidget {},
    //HtmlSlot,
    //QuarzWindow {},
    //NtWindow {},
    //Activity {},
}


pub struct Position {
    x: u32,
    y: u32,
    z: u32,
}


