use ::std::{
    env,
    io::{prelude::*, stderr},
    path::{Path, PathBuf},
    process::{exit, Command},
};

fn main() {
    let mut command = Command::new(env::var("MERCURIAL_PATH").map(PathBuf::from).unwrap_or_default().join("hg"));

    if Path::new("dyncall").exists() {
        command.current_dir("./dyncall");
        command.arg("update");
    } else {
        command.args(
            &[
                "clone",
                "https://dyncall.org/pub/dyncall/dyncall/",
            ][..],
        );
    }

    let output = command.output().unwrap();
match output.status.code() {
        Some(x) if x > 0 => {
            let mut err = stderr();

            write!(err, "stderr: ").unwrap();
            err.write_all(&output.stderr[..]).unwrap();

            drop((err, output));

            exit(x)
        }
        _ => {}
    }

    cc::Build::new()
        .file("dyncall_ext.cpp")
        .compile("dyncall_ext");
}
