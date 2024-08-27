use std::path::{Path, PathBuf};

use qt_core::{qs, QString, QTimer, SlotNoArgs};
use qt_widgets::{QApplication, QGridLayout, QWidget};
use qt_gui::{cpp_core::CppBox};

use crate::{error::MmeResult, implementors::{html::HtmlPresenter, qt_widget::QtWidgetSlot}, presenter};
use crate::slot::{Slot, SlotTrait};
use crate::presenter::Presenter;


pub struct Mme {

}

unsafe fn qstring_to_rust(q_string: CppBox<QString>) -> String {
    let mut rust_string = String::new();
    let q_string_size = q_string.size();

    for j in 0..q_string_size {
        let q_char = q_string.index_int(j);
        let rust_char = char::from_u32(q_char.unicode() as u32);
        rust_string.push(rust_char.unwrap());
    }
    return rust_string;
}

impl Mme {
    pub fn new() -> MmeResult<Mme> {
        Ok(Mme {})
    }

    pub fn create_x_window(self) -> MmeResult<()> {
        unsafe {

            let backend = i_slint_backend_qt::Backend::new();


            let main_widget = qt_widgets::QWidget::new_0a();
            main_widget.show();
            main_widget.set_window_title(&qs("mme_main"));



            comandr::platform::qt::init(main_widget.as_ptr());
          
            //let other_window = OtherWindow::new().unwrap();
            //other_window.show();

            let presenter: Presenter = Presenter::HtmlPresenter(HtmlPresenter::from_folder(Path::new("/home/me/work/mme/mme-presenters/presenters/hello-world").to_owned())?);

            let mut slot: Slot = Slot::QtWidgetSlot(QtWidgetSlot::from_widget(main_widget)?);

            slot.load(presenter);


            println!("run_event_loop");
            unsafe {
                qt_core::QCoreApplication::exec();
            }
            //backend.run_event_loop();
            //unsafe {
                //run_my_event_loop(my_app);
            //}

            Ok(())
        }
    }
}



unsafe fn test(main_widget: QWidget) {



    let widgets = QApplication::top_level_widgets();
    //let widget_iter = (*widgets).begin_mut();

    let widgets_size = widgets.size();
    for i in 0..widgets_size {
        let widget = *widgets.index(i);
        println!("widget: {:?}", widget);
        println!("hidden: {}", (*widget).is_hidden());
        //(*widget).set_hidden(false);


        let widget_title = qstring_to_rust((*widget).window_title());
        println!("title: {}", widget_title);

        println!("");
    }

    let window = MainWindow::new().unwrap();
    let hi = window.window();
    window.show();
}

slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}
