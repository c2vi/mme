use std::path::PathBuf;

use crate::{error::MmeResult, slot::SlotTrait};


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
