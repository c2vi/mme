use std::path::{Path, PathBuf};

use crate::{error::MmeResult, implementors::{html::HtmlPresenter}, presenter};
use crate::slot::{Slot, SlotTrait};
use crate::presenter::Presenter;
use tracing::info;
use comandr::Comandr;

#[cfg(features = "os-target")]
use crate::implementors::qt_widget::QtWidgetSlot;
#[cfg(features = "os-target")]
use qt_core::{qs, QString, QTimer, SlotNoArgs};
#[cfg(features = "os-target")]
use qt_widgets::{QApplication, QGridLayout, QWidget};
#[cfg(features = "os-target")]
use qt_gui::{cpp_core::CppBox};

use mize::Module;
use mize::MizeResult;
use mize::Instance;
use mize::mize_err;
use mize::MizeError;

pub struct Mme {
    pub comandr: Comandr,
    pub hi: String,
}

#[no_mangle]
extern "C" fn get_mize_module_mme(empty_module: &mut Box<dyn Module + Send + Sync>) -> () {
    let comandr = Comandr::new();
    let new_box: Box<dyn Module + Send + Sync> = Box::new( Mme { comandr, hi: "hi inside mme".to_owned() } );

    *empty_module = new_box
}

impl Module for Mme {
    fn init(&mut self, _instance: &Instance) -> MizeResult<()> {
        println!("MmeModule init");

        #[cfg(feature = "os-target")]
        {
            self.create_x_window().map_err(|e| mize_err!("From MmeError: {:?}", e))?;
        }

        #[cfg(feature = "wasm-target")]
        {
            //ther is no html slots yet
            //self.create_html_slot().map_err(|e| mize_err!("From MmeError: {:?}", e))
        }

        Ok(())
    }

    fn exit(&mut self, _instance: &Instance) -> MizeResult<()> {
        info!("mme module exit");
        Ok(())
    }
    
}

#[cfg(feature = "qt")]
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
        let comandr = Comandr::new();
        Ok(Mme { comandr, hi: "hi inside mme".to_owned() })
    }

    #[cfg(feature = "wasm-target")]
    pub fn create_html_slot() -> MmeResult<()> {
        println!("hi wasm");
        Ok(())
    }

    #[cfg(feature = "os-target")]
    pub fn create_x_window(&self) -> MmeResult<()> {
        use tao::{
            event::{Event, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        };
        use wry::{WebViewBuilder, WebViewExtUnix};
        use webkit2gtk::{Settings, WebInspectorExt};
        use webkit2gtk::WebViewExt;

        fn webui() -> wry::Result<()> {
            let event_loop = EventLoop::new();
            let window = WindowBuilder::new().build(&event_loop).unwrap();

            #[cfg(any(
                target_os = "windows",
                target_os = "macos",
                target_os = "ios",
                target_os = "android"
            ))]
            let builder = WebViewBuilder::new(&window);

            #[cfg(not(any(
                target_os = "windows",
                target_os = "macos",
                target_os = "ios",
                target_os = "android"
            )))]

            let builder = {
                use tao::platform::unix::WindowExtUnix;
                use wry::WebViewBuilderExtUnix;
                let vbox = window.default_vbox().unwrap();
                WebViewBuilder::new_gtk(vbox)
            };

            let _webview = builder
            //.with_url("file:///home/me/work/mme/test.html")
            .with_url("file:///home/me/work/mme-presenters/presenters/mme-js/dist/index.html")
            /*
            .with_drag_drop_handler(|e| {
              match e {
                wry::DragDropEvent::Enter { paths, position } => {
                  println!("DragEnter: {position:?} {paths:?} ")
                }
                wry::DragDropEvent::Over { position } => println!("DragOver: {position:?} "),
                wry::DragDropEvent::Drop { paths, position } => {
                  println!("DragDrop: {position:?} {paths:?} ")
                }
                wry::DragDropEvent::Leave => println!("DragLeave"),
                _ => {}
              }

              true
            })
            */
            .build()?;
            //_webview.open_devtools();

            let settings = Settings::builder()
                .allow_file_access_from_file_urls(true)
                .enable_developer_extras(true)
                .build();
            let __webview = _webview.webview();
           __webview.set_settings(&settings);

            let inspector = __webview.inspector().expect("no inspector");
            inspector.show();

            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Wait;

            if let Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } = event
            {
              *control_flow = ControlFlow::Exit
            }
            });
        }

        Ok(webui()?)
    }

    #[cfg(features = "qt")]
    pub fn create_qt_slot(&self) -> MmeResult<()> {
        unsafe {

            let backend = i_slint_backend_qt::Backend::new();


            let main_widget = qt_widgets::QWidget::new_0a();
            main_widget.show();
            main_widget.set_window_title(&qs("mme_main"));



            comandr::platform::qt::init(main_widget.as_ptr());
          
            //let other_window = OtherWindow::new().unwrap();
            //other_window.show();

            let presenter: Presenter = Presenter::HtmlPresenter(HtmlPresenter::from_folder(Path::new("/home/me/work/mme-presenters/presenters/hello-world").to_owned())?);

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



