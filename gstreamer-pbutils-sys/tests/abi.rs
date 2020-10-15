// This file was generated by gir (https://github.com/gtk-rs/gir @ ad40c01)
// from gir-files (https://github.com/gtk-rs/gir-files @ 18c9a37)
// DO NOT EDIT

extern crate gstreamer_pbutils_sys;
extern crate shell_words;
extern crate tempfile;
use gstreamer_pbutils_sys::*;
use std::env;
use std::error::Error;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["gstreamer-pbutils-1.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn define<'a, V: Into<Option<&'a str>>>(&mut self, var: &str, val: V) {
        let arg = match val.into() {
            None => format!("-D{}", var),
            Some(val) => format!("-D{}={}", var, val),
        };
        self.args.push(arg);
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let mut cmd = Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
    /// Number of tests that failed to compile.
    failed_to_compile: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn record_failed_to_compile(&mut self) {
        self.failed += 1;
        self.failed_to_compile += 1;
    }
    fn summary(&self) -> String {
        format!(
            "{} passed; {} failed (compilation errors: {})",
            self.passed, self.failed, self.failed_to_compile
        )
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let tmpdir = Builder::new()
        .prefix("abi")
        .tempdir()
        .expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(
        "1",
        get_c_value(tmpdir.path(), &cc, "1").expect("C constant"),
        "failed to obtain correct constant value for 1"
    );

    let mut results: Results = Default::default();
    for (i, &(name, rust_value)) in RUST_CONSTANTS.iter().enumerate() {
        match get_c_value(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            }
            Ok(ref c_value) => {
                if rust_value == c_value {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!(
                        "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                        name, rust_value, c_value
                    );
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("constants ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let tmpdir = Builder::new()
        .prefix("abi")
        .tempdir()
        .expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(
        Layout {
            size: 1,
            alignment: 1
        },
        get_c_layout(tmpdir.path(), &cc, "char").expect("C layout"),
        "failed to obtain correct layout for char type"
    );

    let mut results: Results = Default::default();
    for (i, &(name, rust_layout)) in RUST_LAYOUTS.iter().enumerate() {
        match get_c_layout(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            }
            Ok(c_layout) => {
                if rust_layout == c_layout {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!(
                        "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                        name, rust_layout, &c_layout
                    );
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("layout    ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

fn get_c_layout(dir: &Path, cc: &Compiler, name: &str) -> Result<Layout, Box<dyn Error>> {
    let exe = dir.join("layout");
    let mut cc = cc.clone();
    cc.define("ABI_TYPE_NAME", name);
    cc.compile(Path::new("tests/layout.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    let stdout = str::from_utf8(&output.stdout)?;
    let mut words = stdout.trim().split_whitespace();
    let size = words.next().unwrap().parse().unwrap();
    let alignment = words.next().unwrap().parse().unwrap();
    Ok(Layout { size, alignment })
}

fn get_c_value(dir: &Path, cc: &Compiler, name: &str) -> Result<String, Box<dyn Error>> {
    let exe = dir.join("constant");
    let mut cc = cc.clone();
    cc.define("ABI_CONSTANT_NAME", name);
    cc.compile(Path::new("tests/constant.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    let output = str::from_utf8(&output.stdout)?.trim();
    if !output.starts_with("###gir test###") || !output.ends_with("###gir test###") {
        return Err(format!(
            "command {:?} return invalid output, {:?}",
            &abi_cmd, &output
        )
        .into());
    }

    Ok(String::from(&output[14..(output.len() - 14)]))
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "GstAudioVisualizer",
        Layout {
            size: size_of::<GstAudioVisualizer>(),
            alignment: align_of::<GstAudioVisualizer>(),
        },
    ),
    (
        "GstAudioVisualizerClass",
        Layout {
            size: size_of::<GstAudioVisualizerClass>(),
            alignment: align_of::<GstAudioVisualizerClass>(),
        },
    ),
    (
        "GstAudioVisualizerShader",
        Layout {
            size: size_of::<GstAudioVisualizerShader>(),
            alignment: align_of::<GstAudioVisualizerShader>(),
        },
    ),
    (
        "GstDiscoverer",
        Layout {
            size: size_of::<GstDiscoverer>(),
            alignment: align_of::<GstDiscoverer>(),
        },
    ),
    (
        "GstDiscovererAudioInfoClass",
        Layout {
            size: size_of::<GstDiscovererAudioInfoClass>(),
            alignment: align_of::<GstDiscovererAudioInfoClass>(),
        },
    ),
    (
        "GstDiscovererClass",
        Layout {
            size: size_of::<GstDiscovererClass>(),
            alignment: align_of::<GstDiscovererClass>(),
        },
    ),
    (
        "GstDiscovererContainerInfoClass",
        Layout {
            size: size_of::<GstDiscovererContainerInfoClass>(),
            alignment: align_of::<GstDiscovererContainerInfoClass>(),
        },
    ),
    (
        "GstDiscovererInfoClass",
        Layout {
            size: size_of::<GstDiscovererInfoClass>(),
            alignment: align_of::<GstDiscovererInfoClass>(),
        },
    ),
    (
        "GstDiscovererResult",
        Layout {
            size: size_of::<GstDiscovererResult>(),
            alignment: align_of::<GstDiscovererResult>(),
        },
    ),
    (
        "GstDiscovererSerializeFlags",
        Layout {
            size: size_of::<GstDiscovererSerializeFlags>(),
            alignment: align_of::<GstDiscovererSerializeFlags>(),
        },
    ),
    (
        "GstDiscovererStreamInfoClass",
        Layout {
            size: size_of::<GstDiscovererStreamInfoClass>(),
            alignment: align_of::<GstDiscovererStreamInfoClass>(),
        },
    ),
    (
        "GstDiscovererSubtitleInfoClass",
        Layout {
            size: size_of::<GstDiscovererSubtitleInfoClass>(),
            alignment: align_of::<GstDiscovererSubtitleInfoClass>(),
        },
    ),
    (
        "GstDiscovererVideoInfoClass",
        Layout {
            size: size_of::<GstDiscovererVideoInfoClass>(),
            alignment: align_of::<GstDiscovererVideoInfoClass>(),
        },
    ),
    (
        "GstEncodingTargetClass",
        Layout {
            size: size_of::<GstEncodingTargetClass>(),
            alignment: align_of::<GstEncodingTargetClass>(),
        },
    ),
    (
        "GstInstallPluginsReturn",
        Layout {
            size: size_of::<GstInstallPluginsReturn>(),
            alignment: align_of::<GstInstallPluginsReturn>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) GST_AUDIO_VISUALIZER_SHADER_FADE", "1"),
    ("(gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_DOWN", "3"),
    (
        "(gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_HORIZ_IN",
        "7",
    ),
    (
        "(gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_HORIZ_OUT",
        "6",
    ),
    ("(gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_LEFT", "4"),
    (
        "(gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_RIGHT",
        "5",
    ),
    ("(gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_UP", "2"),
    (
        "(gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_VERT_IN",
        "9",
    ),
    (
        "(gint) GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_VERT_OUT",
        "8",
    ),
    ("(gint) GST_AUDIO_VISUALIZER_SHADER_NONE", "0"),
    ("(gint) GST_DISCOVERER_BUSY", "4"),
    ("(gint) GST_DISCOVERER_ERROR", "2"),
    ("(gint) GST_DISCOVERER_MISSING_PLUGINS", "5"),
    ("(gint) GST_DISCOVERER_OK", "0"),
    ("(guint) GST_DISCOVERER_SERIALIZE_ALL", "7"),
    ("(guint) GST_DISCOVERER_SERIALIZE_BASIC", "0"),
    ("(guint) GST_DISCOVERER_SERIALIZE_CAPS", "1"),
    ("(guint) GST_DISCOVERER_SERIALIZE_MISC", "4"),
    ("(guint) GST_DISCOVERER_SERIALIZE_TAGS", "2"),
    ("(gint) GST_DISCOVERER_TIMEOUT", "3"),
    ("(gint) GST_DISCOVERER_URI_INVALID", "1"),
    ("GST_ENCODING_CATEGORY_CAPTURE", "capture"),
    ("GST_ENCODING_CATEGORY_DEVICE", "device"),
    ("GST_ENCODING_CATEGORY_FILE_EXTENSION", "file-extension"),
    ("GST_ENCODING_CATEGORY_ONLINE_SERVICE", "online-service"),
    ("GST_ENCODING_CATEGORY_STORAGE_EDITING", "storage-editing"),
    ("(gint) GST_INSTALL_PLUGINS_CRASHED", "100"),
    ("(gint) GST_INSTALL_PLUGINS_ERROR", "2"),
    ("(gint) GST_INSTALL_PLUGINS_HELPER_MISSING", "202"),
    ("(gint) GST_INSTALL_PLUGINS_INSTALL_IN_PROGRESS", "203"),
    ("(gint) GST_INSTALL_PLUGINS_INTERNAL_FAILURE", "201"),
    ("(gint) GST_INSTALL_PLUGINS_INVALID", "101"),
    ("(gint) GST_INSTALL_PLUGINS_NOT_FOUND", "1"),
    ("(gint) GST_INSTALL_PLUGINS_PARTIAL_SUCCESS", "3"),
    ("(gint) GST_INSTALL_PLUGINS_STARTED_OK", "200"),
    ("(gint) GST_INSTALL_PLUGINS_SUCCESS", "0"),
    ("(gint) GST_INSTALL_PLUGINS_USER_ABORT", "4"),
];
