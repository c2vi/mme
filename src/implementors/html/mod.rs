use std::path::PathBuf;

use crate::error::MmeResult;


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
