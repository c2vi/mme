
use std::path::PathBuf;
use comandr::Comandr;

use crate::slot::SlotTrait;
use crate::mme::Mme;
use mize::MizeResult;

pub mod webview_con;


pub struct HtmlPresenter {
    pub path: PathBuf,
}

impl HtmlPresenter {
    pub fn from_folder(path: PathBuf) -> MizeResult<HtmlPresenter> {
        Ok(HtmlPresenter { path })
    }
    
}

pub struct HtmlSlot {
}

impl SlotTrait for HtmlSlot {
    fn load(&mut self,presenter:crate::presenter::Presenter) -> MizeResult<()> {
        Ok(())
    }
}


#[cfg(feature = "wasm-target")]
mod wasm {

    use comandr::Comandr;
    use comandr::Command;
    use web_sys::console;
    use web_sys::EventTarget;
    use web_sys::js_sys::Function;
    use std::panic;
    use std::ptr::NonNull;
    use std::slice::Iter;
    use comandr::Module;
    use comandr::ComandrResult;

    use crate::mme::Mme;

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
    // end of console log

    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    #[cfg(feature = "wee_alloc")]
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

    #[wasm_bindgen]
    pub struct MmeJs {
        inner: NonNull<Mme>,
        //inner: *mut Mme,
    }

    #[wasm_bindgen]
    impl MmeJs {
        #[wasm_bindgen(constructor)]
        pub fn new() -> MmeJs {

            panic::set_hook(Box::new(console_error_panic_hook::hook));

            /*
            if let Ok(mut mme) = Mme::new() {
                mme.comandr.init();
                let mme_comandr_module = Box::new(MmeComandrModule::new());
                mme.comandr.load_module(mme_comandr_module);
                let mut mmejs = MmeJs { inner: NonNull::from(Box::leak(Box::new(mme))) };
                return mmejs;
            } else {
                console_log!("Mme::new() error");
                panic!("Mme::new() failed");
            }
            */
            panic!("Mme::new() failed");
        }

        #[wasm_bindgen]
        pub unsafe fn comandr_search(&mut self, string: String) -> Vec<String> {
            //self.inner.as_mut().comandr.search(string)
            Vec::new()
        }

        pub unsafe fn comandr_list(&mut self) -> Vec<String> {
            //self.inner.as_mut().comandr.list_commands()
            Vec::new()
        }

        #[wasm_bindgen]
        pub unsafe fn comandr_run(&mut self, name: String, args: Vec<String>) -> () {
            //self.inner.as_mut().comandr.execute(name, args)

        }

    }


    pub struct MmeComandrModule {
        commands: Vec<Command>
    }

    fn reload_page() -> ComandrResult<()>{
        let js_fn_str = r#"
            window.location.reload()
        "#;
        let js_fn = Function::new_no_args(js_fn_str);
    
        js_fn.call0(&web_sys::wasm_bindgen::JsValue::NULL);

        Ok(())
    }

    fn hello() -> ComandrResult<()> {
        console_log!("hellooooo command");
        Ok(())
    }

    impl MmeComandrModule {
        pub fn new() -> MmeComandrModule {
            let commands = vec![
                Command { name: "reload".to_owned(), closure: Box::new(reload_page) },
                Command { name: "test".to_owned(), closure: Box::new(hello) },
            ];
            MmeComandrModule { commands }
        }
    }

    impl Module for MmeComandrModule {
        fn name(&self) -> String {
            "mme".to_owned()
        }

        fn commands(&self) -> Iter<'_, Command> {
            self.commands.iter()
        }

        fn get_command(&mut self, name: String) -> Option<&mut Command> {
            for command in self.commands.iter_mut() {
                if command.name == name {
                    return Some(command);
                }
            }
            return None;
        }
    }

}



