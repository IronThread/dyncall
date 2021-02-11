use ::std::{
    io::{prelude::*, stderr},
    path::Path,
    process::{exit, Command, Output},
};

fn main() {
    let mut command = Command::new("python");

    command.current_dir("./mercurial");
    command.args(
        &[
            "hg",
            "update"
        ][..],
    );

    let output = command.output().unwrap();

    let handle_err = |output: Output| match output.status.code() {
        Some(x) if x > 0 => {
            let mut err = stderr();

            write!(err, "stderr: ").unwrap();
            err.write_all(&output.stderr[..]).unwrap();

            drop((err, output));

            exit(x)
        }
        _ => {}
    };

    handle_err(output);

    let mut command = Command::new("python");

    if Path::new("dyncall").exists() {
        command.current_dir("./dyncall");
        command.args(&["../mercurial/hg", "update"][..]);
    } else {
        command.args(
            &[
                "mercurial/hg",
                "clone",
                "https://dyncall.org/pub/dyncall/dyncall/",
            ][..],
        );
    }

    let output = command.output().unwrap();

    handle_err(output);

    cc::Build::new()
        .file("dyncall_ext.cpp")
        .compile("dyncall_ext");
}
