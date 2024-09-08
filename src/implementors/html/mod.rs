
use std::path::PathBuf;
use crate::error::MmeError;
use comandr::Comandr;

use crate::{error::MmeResult, slot::SlotTrait};
use crate::mme::Mme;



pub struct HtmlPresenter {
    pub path: PathBuf,
}

impl HtmlPresenter {
    pub fn from_folder(path: PathBuf) -> MmeResult<HtmlPresenter> {
        Ok(HtmlPresenter { path })
    }
    
}

pub struct HtmlSlot {
}

impl SlotTrait for HtmlSlot {
    fn load(&mut self,presenter:crate::presenter::Presenter) -> MmeResult<()> {
        Ok(())
    }
}


#[cfg(feature = "wasm-target")]
mod wasm {

    use comandr::Comandr;
    use wasm_bindgen::prelude::*;
    use web_sys::console;
    use web_sys::EventTarget;
    use web_sys::js_sys::Function;
    use std::panic;
    use std::ptr::NonNull;

    use crate::mme::Mme;

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

            if let Ok(mut mme) = Mme::new() {
                mme.comandr.init();
                let mut mmejs = MmeJs { inner: NonNull::from(Box::leak(Box::new(mme))) };
                return mmejs;
            } else {
                console_log!("Mme::new() error");
                panic!("Mme::new() failed");
            }
        }

        #[wasm_bindgen]
        pub unsafe fn comandr_search(&mut self, string: String) -> Vec<String> {
            self.inner.as_mut().comandr.search(string)
        }

    }

    impl Mme {
        pub fn test(&self) {
            console_log!("mme test fn: {}", self.hi);
        }
    }



    //custom_print::define_macros!({ jprint, jprintln },
       //concat, extern "C" fn console_log(_: *const u8, _: usize));
    //custom_print::define_macros!({ jeprint, jeprintln, jdbg },
       //concat, extern "C" fn console_warn(_: *const u8, _: usize));
    //custom_print::define_init_panic_hook!(
       //concat, extern "C" fn console_error(_: *const u8, _: usize));


}



