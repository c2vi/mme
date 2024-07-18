#![ allow( warnings ) ]

use std::path::PathBuf;
use clap::{ArgAction, ArgMatches};
use clap::{Arg, crate_version, Command};
use cpp::cpp;

use std::sync::Arc;
use std::io::Write;
use colored::Colorize;
use std::env;
use tracing_subscriber::fmt::Subscriber;
use tracing::Level;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::EnvFilter;
use crate::logging::init_logger;
use mme::error::{MmeResult, MmeError};
use mme::mme::Mme;

use tracing::{trace, debug, info, warn, error};

mod cli {
}

mod logging;

static APPNAME: &str = "mme";
static DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::WARN;

fn main() {
    let cli_matches = cli_matches();

    init_logger(&cli_matches);

    // match command
    let result = match cli_matches.subcommand() {
        // mi daemon
        //Some(("run", sub_matches)) => cli::run(sub_matches),
        // some unknown command passed

        Some((cmd, sub_matches)) => Err(MmeError::new().msg(format!("The subcommand: {} is not known. use --help to list availavle commands", cmd))),

        None => default_cmd(),
    };

    if let Err(err) = result {
        err.log();
    }
}

#[link(name = "mme")]
extern "C" { 
    fn hello() -> ();
}

fn default_cmd() -> MmeResult<()> {
    let mme = Mme::new_x_window();


    unsafe {
        hello()
    }

    //let window = MainWindow::new().unwrap();
    //window.run().unwrap();
    Ok(())
}



slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}


fn cli_matches() -> clap::ArgMatches {


    let main = Command::new(APPNAME)
        .version(crate_version!())
        .author("Sebastian Moser")
        .about("The Main Mize Explorer or Mize Ui Framework")
        .arg(Arg::new("verbose")
            .long("verbose")
            .short('v')
            .action(ArgAction::Count)
            .global(true)
        )
        .arg(Arg::new("log-level")
            .long("log-level")
            .value_name("LOGLEVEL")
            .help("set the log-level to one of OFF, ERROR, WARN, INFO, DEBUG, TRACE")
            .global(true)
        )
        .arg(Arg::new("silent")
            .long("silent")
            .action(ArgAction::SetTrue)
            .help("set the log-level to OFF")
            .global(true)
        )
        .arg(Arg::new("folder")
            .short('f')
            .long("folder")
            .help("The folder the Instance stores all it's data and the socket for connections")
            .global(true)
        )
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .help("overwrite config options")
            .global(true)
        )
        .arg(Arg::new("config-file")
            .long("config-file")
            .help("specify a config file")
            .global(true)
        );

    return main.get_matches();
}



use tao::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};
use wry::WebViewBuilder;

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
    .with_url("http://c2vi.dev")
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
