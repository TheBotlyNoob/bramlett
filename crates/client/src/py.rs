use std::collections::HashMap;

use common::GameId;
use rustpython_vm as vm;
use tokio::sync::{mpsc, oneshot};
use vm::py_compile;

use crate::Ctx;

pub enum Error {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequestTy {
    PostInstall,
    PreRun,
    PostRun,
}
impl RequestTy {
    const fn func(self) -> &'static str {
        match self {
            Self::PostInstall => "post_install",
            Self::PreRun => "pre_run",
            Self::PostRun => "post_run",
        }
    }
}

pub struct Request {
    pub ty: RequestTy,
    pub id: GameId,
    pub finish: oneshot::Sender<Result<(), Error>>,
}

#[allow(
    clippy::module_name_repetitions,
    clippy::missing_panics_doc,
    clippy::needless_pass_by_value
)]
pub fn py_loop(mut rx: mpsc::UnboundedReceiver<Request>, ctx: Ctx) -> ! {
    let interp = vm::Interpreter::with_init(vm::Settings::default(), |vm| {
        vm.add_native_modules(vm::stdlib::get_module_inits());
        let platform_dirs_mod = py_compile!(
            source = "platformdirs",
            mode = "exec",
            module_name = "platformdirs"
        );
        vm.add_native_modules(platform_dirs_mod);
    });

    let mut scope_map = HashMap::new();

    interp.enter(|vm| loop {
        let req = rx.blocking_recv().unwrap();

        let scope = scope_map.entry(req.id).or_insert_with(|| {
            let scope = vm.new_scope_with_builtins();

            let game = ctx.config.games.get(&req.id).unwrap().clone();

            vm.run_code_obj(
                vm.compile(
                    &game.info.hooks,
                    vm::compiler::Mode::Exec,
                    format!("hooks-for-{}.py", game.info.id),
                )
                .unwrap(),
                scope.clone(),
            )
            .unwrap();
            scope
        });

        let func = req.ty.func();
    })
}
