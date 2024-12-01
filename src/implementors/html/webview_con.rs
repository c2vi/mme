use wry::WebView;
use base64::Engine;
use crossbeam::channel::Receiver;
use mize::{mize_err, MizeResult};
use mize::proto::MizeMessage;
use crossbeam::channel::Sender;
use base64::engine::general_purpose::STANDARD;
use wry::http::Request;

use crate::mme::Mme;

pub fn ipc_handler(req: Request<String>, mme: Mme, conn_id: u64) {
    let msg_str = req.body();

    if let Ok(msg_bytes) = STANDARD.decode(msg_str) {
        let value: CborValue = ciborium::from_reader(msg_bytes)?;
        let msg = MizeMessage::new(value, conn_id);
        trace!("webview_con incoming got msg: {}", msg);
        instance.got_msg(msg);
    } else {
        mme.mize.report_err(mize_err!("Base64 Decoding of a message from the Webview to the host instance failed"));
        return;
    }
}

pub fn mme_setup_weview_con_host(mme: &mut Mme, rx: Receiver<MizeMessage>, webview: &WebView) -> MizeResult<()> {

    let mize_clone = mme.mize.clone();
    mme.mize.spawn("webview-con-outgoing", || {
        for msg in rx {
            let msg_bytes = Vec::new();
            let res = ciborium::into_writer(&value, msg_bytes);
            if let Err(err) = res {
                mize_clone.report_err(err);
                continue;
            }
            let msg_string = STANDARD.encode(msg_bytes);
            webview.evaluate_script(format!("mize.mod.mme.msg_recv_fn({})", msg_string).as_str())
        }
        Ok(())
    });

    return Ok(());
}

pub fn mme_setup_weview_con_js(mme: &mut Mme) -> MizeResult<Sender<MizeMessage>> {
    let (tx, rx): (Sender<MizeMessage>, Receiver<MizeMessage>) = crossbeam::channel::unbounded();

    return Ok(tx);
}
