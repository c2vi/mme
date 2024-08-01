use std::path::{Path, PathBuf};
use slint_interpreter::{ComponentInstance, ComponentDefinition, ComponentCompiler, Value, SharedString, ComponentHandle};

use crate::space::{Space, SpaceImplementor, SpaceTrait, Position};
use crate::error::MmeResult;

//pub mod qt_backend;



pub struct SlintWidget {
    inner: ComponentInstance,
}


impl SlintWidget {
    fn from_slint_file(path: PathBuf) -> MmeResult<SlintWidget> {
        todo!()
    }

    fn sample() -> MmeResult<SlintWidget> {

        let code = r#"
            export component MyWin inherits Window {
                in property <string> my_name;
                Text {
                    text: "Hello, " + my_name;
                }
            }
        "#;

        let mut compiler = ComponentCompiler::default();
        let definition = spin_on::spin_on(compiler.build_from_source(code.into(), Default::default()));
        assert!(compiler.diagnostics().is_empty());
        let instance = definition.unwrap().create().unwrap();
        instance.set_property("my_name", Value::from(SharedString::from("World"))).unwrap();
        return SlintWidget::from_instance(instance);
    }

    fn from_instance(instance: ComponentInstance) -> MmeResult<SlintWidget> {
        Ok(SlintWidget {
            inner: instance,
        })
    }
}

impl SpaceTrait for SlintWidget {
    fn put_top(self, pos: Position, widget: Space) -> MmeResult<()> {
        todo!()
    }
    fn put_top_full(self, widget: Space) -> MmeResult<()> {
        todo!()
    }
}





