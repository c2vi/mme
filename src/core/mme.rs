use crate::error::MmeResult;

use qt_core::{qs, QString, QTimer, SlotNoArgs};
use qt_widgets::{QApplication, QGridLayout, QWidget};
use qt_gui::cpp_core::CppBox;


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

            //let core_app = unsafe { qt_core::QCoreApplication::instance() };
            //let app = core_app.dynamic_cast::<QApplication>();

            let main_widget = qt_widgets::QWidget::new_0a();
            main_widget.show();
            main_widget.set_window_title(&qs("mme_main"));




    let grid_layout = unsafe { qt_widgets::QGridLayout::new_0a() };
    main_widget.set_layout(&grid_layout);

    let button = unsafe { qt_widgets::QPushButton::new() };
    button.set_text(&qs("hello from rust button"));

    let button2 = unsafe { qt_widgets::QPushButton::new() };
    button2.set_text(&qs("hello from another button"));
    
    let to_place_slint = QWidget::new_0a();
    let to_place_slint_layout = QGridLayout::new_0a();
    to_place_slint.set_layout(&to_place_slint_layout);
    to_place_slint.set_window_title(&qs("place_slint"));
    to_place_slint.set_style_sheet(&qs("border: 1px solid red"));
    

    grid_layout.add_widget_3a(&button, 0, 1);
    grid_layout.add_widget_3a(&button2, 1, 0);
    grid_layout.add_widget_3a(&to_place_slint, 2, 0);




            comandr::platform::qt::init(to_place_slint.as_ptr());
          
            //let other_window = OtherWindow::new().unwrap();
            //other_window.show();





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
