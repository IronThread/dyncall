use ::std::{
    env,
    fs,
    io::{prelude::*, stderr},
    path::PathBuf,
    process::{exit, Command},
};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dyncall_path = out_dir.join("dyncall");

    let mut command = Command::new(
        env::var("MERCURIAL_PATH")
            .map(PathBuf::from)
            .unwrap_or_default()
            .join("hg"),
    );

    if dyncall_path.exists() {
        command.current_dir(&dyncall_path);
        command.arg("update");
    } else {
        command.current_dir(&out_dir);
        command.args(&["clone", "https://dyncall.org/pub/dyncall/dyncall/"][..]);
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

    let dyncall_ext_path = out_dir.join("dyncall_ext.cpp");

    fs::copy("dyncall_ext.cpp", &dyncall_ext_path).unwrap();

    cc::Build::new()
        .file(&dyncall_ext_path)
        .compile("dyncall_ext");
}
