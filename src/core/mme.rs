use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::Arc;

use crate::{implementors::{html::HtmlPresenter}, presenter};
use crate::slot::{Slot, SlotTrait};
use crate::presenter::Presenter;
use tracing::info;
use comandr::Comandr;
use flume::{ Receiver, Sender };
use base64::engine::general_purpose::STANDARD;
use base64::Engine;


#[cfg(feature = "wasm-target")]
use wasm_bindgen::JsValue;
#[cfg(feature = "wasm-target")]
use web_sys::js_sys::Function;

//#[cfg(feature = "os-target")]
//use crate::implementors::qt_widget::QtWidgetSlot;
#[cfg(feature = "qt")]
use qt_core::{qs, QString, QTimer, SlotNoArgs};
#[cfg(feature = "qt")]
use qt_widgets::{QApplication, QGridLayout, QWidget};
#[cfg(feature = "qt")]
use qt_gui::{cpp_core::CppBox};


use mize::Module;
use mize::MizeResult;
use mize::Instance;
use mize::mize_err;
use mize::MizeError;
use mize::proto::MizeMessage;

#[derive(Clone)]
pub struct Mme {
    pub comandr: Arc<Mutex<Comandr>>,
    pub mize: Instance,
}

#[no_mangle]
extern "C" fn get_mize_module_mme(empty_module: &mut Box<dyn Module + Send + Sync>, mize: Instance) -> () {
    let comandr = Comandr::new();
    let new_box: Box<dyn Module + Send + Sync> = Box::new( Mme { comandr: Arc::new(Mutex::new(comandr)), mize, } );

    *empty_module = new_box
}

impl Module for Mme {
    fn init(&mut self, _instance: &Instance) -> MizeResult<()> {


    #[cfg(feature = "wasm-target")]
    {
    // console log
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }
    macro_rules! console_log {
        // Note that this is using the `log` function imported above during
        // `bare_bones`
        ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
    }
    console_log!("mme module inittttttttttttttttttttttt");
    }

        println!("MmeModule init");

        #[cfg(feature = "os-target")]
        {
            //self.mize.spawn("mme-main", || self.create_x_window());
            let mut cloned_self = self.clone();
            // "significant cross-platform compatibility hazard." xD
            //self.mize.spawn("mme-main", move || cloned_self.create_x_window());
            
            self.create_x_window();

        }

        #[cfg(feature = "wasm-target")]
        {
            use web_sys::js_sys::eval;
            use crate::implementors::html::wasm::MmeJs;
            use std::ptr::NonNull;

            // create mme module object
            let mut mme_js = MmeJs {
                inner: NonNull::from(Box::leak(Box::new(self.clone()))),
                webview_con_id: 0,
            };

            let func = Function::new_with_args("mme_js", r#"
                window.mize.mod.mme = mme_js;
            "#);
            func.call1(&JsValue::null(), &JsValue::from(mme_js)).map_err(|e| mize_err!("from js error: {:?}", e));

            // 
        }

        Ok(())
    }

    fn exit(&mut self, _instance: &Instance) -> MizeResult<()> {
        info!("mme module exit");
        Ok(())
    }

    fn clone_module(&self) -> Box<dyn Module + Send + Sync> {
        Box::new(self.clone()

            )
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
    pub fn new(mize: Instance) -> MizeResult<Mme> {
        let comandr = Comandr::new();
        Ok(Mme { comandr: Arc::new(Mutex::new(comandr)), mize, })
    }

    #[cfg(feature = "wasm-target")]
    pub fn create_html_slot() -> MizeResult<()> {
        println!("hi wasm");
        Ok(())
    }

    #[cfg(feature = "os-target")]
    pub fn create_x_window(&mut self) -> MizeResult<()> {
        use std::fs;

        use tao::{
            event::{Event, WindowEvent},
            event_loop::{self, ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy},
            window::WindowBuilder,
        };
        use wry::{http::{Request, Response}, WebViewBuilder};

        #[cfg(target_os = "linux")]
        use wry::WebViewExtUnix;

        #[cfg(target_os = "linux")]
        use webkit2gtk::{Settings, WebInspectorExt};

        #[cfg(target_os = "linux")]
        use webkit2gtk::WebViewExt;

        use crate::implementors::html::webview_con::{msg_from_string, msg_to_string};

        let event_loop = EventLoopBuilder::with_user_event().build();
        let event_loop_proxy: EventLoopProxy<MizeMessage> = event_loop.create_proxy();
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

        //let html_str = fs::read_to_string(format!("{}/../implementors/html/js-runtime/dist/index.html", file!()))?;
        //println!("html_str: {}", html_str);

        // get the path of the js-runtime
        let mme_module_path = self.mize.fetch_module("mme")?;
        println!("mme module path: {}", mme_module_path);


        // add the mize connection to the instance inside the webview
        let (tx, rx): (Sender<MizeMessage>, Receiver<MizeMessage>) = flume::unbounded();
        let conn_id = self.mize.new_connection(tx)?;


        let mut self_clone = self.clone();
        let webview = builder
        //.with_url("http://localhost:8000/index.html")
        //.with_url("../implementors/html/js-runtime/dist/index.html")
        .with_url(format!("file://{}/index.html", mme_module_path))
        .with_ipc_handler(move | res: Request<String> | {
            crate::implementors::html::webview_con::ipc_handler(res, self_clone.clone(), conn_id)
        })
        //.with_html(html_str)
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

        #[cfg(target_os = "linux")]
        {
            let settings = Settings::builder()
                .allow_file_access_from_file_urls(true)
                .enable_developer_extras(true)
                .build();
            let __webview = webview.webview();
           __webview.set_settings(&settings);

            let inspector = __webview.inspector().expect("no inspector");
            inspector.show();
        }


        crate::implementors::html::webview_con::mme_setup_weview_con_host(self, rx, event_loop_proxy)?;
        let cloned_self = self.clone();


        // this is where we block the main thread....
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    *control_flow = ControlFlow::Exit
                },

                Event::UserEvent::<MizeMessage>(msg) => {
                    match msg_to_string(msg) {
                        Ok(msg_string) => {
                            webview.evaluate_script(format!("mize.mod.mme.webview_msg_recv_fn({})", msg_string).as_str());
                        },
                        Err(err) => {
                            cloned_self.mize.report_err(err.into());
                        },
                    };
                },

                _ => {},
            }

        });
    }

    #[cfg(features = "qt")]
    pub fn create_qt_slot(&self) -> MizeResult<()> {
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



