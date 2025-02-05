use crate::{
    apple::config::Config,
    env::{Env, ExplicitEnv as _},
    util::cli::{Report, Reportable},
    DuctExpressionExt,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RunError {
    #[error("Failed to deploy app to simulator: {0}")]
    DeployFailed(std::io::Error),
}

impl Reportable for RunError {
    fn report(&self) -> Report {
        match self {
            Self::DeployFailed(err) => Report::error("Failed to deploy app to simulator", err),
        }
    }
}

pub fn run(
    config: &Config,
    env: &Env,
    non_interactive: bool,
    id: &str,
) -> Result<duct::Handle, RunError> {
    println!("Deploying app to device...");

    let app_dir = config
        .export_dir()
        .join(format!("{}_iOS.xcarchive", config.app().name()))
        .join("Products/Applications")
        .join(format!("{}.app", config.app().stylized_name()));
    let cmd = duct::cmd("xcrun", ["simctl", "install", id])
        .vars(env.explicit_env())
        .before_spawn(move |cmd| {
            cmd.arg(&app_dir);
            Ok(())
        });

    let handle = cmd.start().map_err(RunError::DeployFailed)?;

    handle.wait().map_err(RunError::DeployFailed)?;

    let app_id = format!("{}.{}", config.app().reverse_domain(), config.app().name());

    let mut launcher_cmd =
        duct::cmd("xcrun", ["simctl", "launch", id, &app_id]).vars(env.explicit_env());

    if non_interactive {
        launcher_cmd = launcher_cmd.before_spawn(|cmd| {
            cmd.arg("--console");
            Ok(())
        });
    }
    if non_interactive {
        launcher_cmd.start().map_err(RunError::DeployFailed)
    } else {
        launcher_cmd
            .start()
            .map_err(RunError::DeployFailed)?
            .wait()
            .map_err(RunError::DeployFailed)?;

        duct::cmd(
            "xcrun",
            [
                "simctl",
                "spawn",
                id,
                "log",
                "stream",
                "--level",
                "debug",
                "--predicate",
                &format!("subsystem == \"{app_id}\""),
            ],
        )
        .vars(env.explicit_env())
        .start()
        .map_err(RunError::DeployFailed)
    }
}
