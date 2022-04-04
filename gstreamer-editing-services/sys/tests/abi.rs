// Generated by gir (https://github.com/gtk-rs/gir @ 9aa16ead87e1)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3ff4d3275258)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 233e3205cb17)
// DO NOT EDIT

use gstreamer_editing_services_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["gst-editing-services-1.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
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
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
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
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
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
#[cfg(target_os = "linux")]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let value = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse value");
        c_constants.push((name, value));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_value, &c_value
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
#[cfg(target_os = "linux")]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let size = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse size");
        let alignment = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse alignment");
        c_layouts.push((name, Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_layout, &c_layout
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "GESAsset",
        Layout {
            size: size_of::<GESAsset>(),
            alignment: align_of::<GESAsset>(),
        },
    ),
    (
        "GESAssetClass",
        Layout {
            size: size_of::<GESAssetClass>(),
            alignment: align_of::<GESAssetClass>(),
        },
    ),
    (
        "GESAssetLoadingReturn",
        Layout {
            size: size_of::<GESAssetLoadingReturn>(),
            alignment: align_of::<GESAssetLoadingReturn>(),
        },
    ),
    (
        "GESAudioSource",
        Layout {
            size: size_of::<GESAudioSource>(),
            alignment: align_of::<GESAudioSource>(),
        },
    ),
    (
        "GESAudioSourceClass",
        Layout {
            size: size_of::<GESAudioSourceClass>(),
            alignment: align_of::<GESAudioSourceClass>(),
        },
    ),
    (
        "GESAudioTestSource",
        Layout {
            size: size_of::<GESAudioTestSource>(),
            alignment: align_of::<GESAudioTestSource>(),
        },
    ),
    (
        "GESAudioTestSourceClass",
        Layout {
            size: size_of::<GESAudioTestSourceClass>(),
            alignment: align_of::<GESAudioTestSourceClass>(),
        },
    ),
    (
        "GESAudioTrack",
        Layout {
            size: size_of::<GESAudioTrack>(),
            alignment: align_of::<GESAudioTrack>(),
        },
    ),
    (
        "GESAudioTrackClass",
        Layout {
            size: size_of::<GESAudioTrackClass>(),
            alignment: align_of::<GESAudioTrackClass>(),
        },
    ),
    (
        "GESAudioTransition",
        Layout {
            size: size_of::<GESAudioTransition>(),
            alignment: align_of::<GESAudioTransition>(),
        },
    ),
    (
        "GESAudioTransitionClass",
        Layout {
            size: size_of::<GESAudioTransitionClass>(),
            alignment: align_of::<GESAudioTransitionClass>(),
        },
    ),
    (
        "GESAudioUriSource",
        Layout {
            size: size_of::<GESAudioUriSource>(),
            alignment: align_of::<GESAudioUriSource>(),
        },
    ),
    (
        "GESAudioUriSourceClass",
        Layout {
            size: size_of::<GESAudioUriSourceClass>(),
            alignment: align_of::<GESAudioUriSourceClass>(),
        },
    ),
    (
        "GESBaseEffect",
        Layout {
            size: size_of::<GESBaseEffect>(),
            alignment: align_of::<GESBaseEffect>(),
        },
    ),
    (
        "GESBaseEffectClass",
        Layout {
            size: size_of::<GESBaseEffectClass>(),
            alignment: align_of::<GESBaseEffectClass>(),
        },
    ),
    (
        "GESBaseEffectClip",
        Layout {
            size: size_of::<GESBaseEffectClip>(),
            alignment: align_of::<GESBaseEffectClip>(),
        },
    ),
    (
        "GESBaseEffectClipClass",
        Layout {
            size: size_of::<GESBaseEffectClipClass>(),
            alignment: align_of::<GESBaseEffectClipClass>(),
        },
    ),
    (
        "GESBaseTransitionClip",
        Layout {
            size: size_of::<GESBaseTransitionClip>(),
            alignment: align_of::<GESBaseTransitionClip>(),
        },
    ),
    (
        "GESBaseTransitionClipClass",
        Layout {
            size: size_of::<GESBaseTransitionClipClass>(),
            alignment: align_of::<GESBaseTransitionClipClass>(),
        },
    ),
    (
        "GESBaseXmlFormatter",
        Layout {
            size: size_of::<GESBaseXmlFormatter>(),
            alignment: align_of::<GESBaseXmlFormatter>(),
        },
    ),
    (
        "GESBaseXmlFormatterClass",
        Layout {
            size: size_of::<GESBaseXmlFormatterClass>(),
            alignment: align_of::<GESBaseXmlFormatterClass>(),
        },
    ),
    (
        "GESChildrenControlMode",
        Layout {
            size: size_of::<GESChildrenControlMode>(),
            alignment: align_of::<GESChildrenControlMode>(),
        },
    ),
    (
        "GESClip",
        Layout {
            size: size_of::<GESClip>(),
            alignment: align_of::<GESClip>(),
        },
    ),
    (
        "GESClipAsset",
        Layout {
            size: size_of::<GESClipAsset>(),
            alignment: align_of::<GESClipAsset>(),
        },
    ),
    (
        "GESClipAssetClass",
        Layout {
            size: size_of::<GESClipAssetClass>(),
            alignment: align_of::<GESClipAssetClass>(),
        },
    ),
    (
        "GESClipClass",
        Layout {
            size: size_of::<GESClipClass>(),
            alignment: align_of::<GESClipClass>(),
        },
    ),
    (
        "GESCommandLineFormatter",
        Layout {
            size: size_of::<GESCommandLineFormatter>(),
            alignment: align_of::<GESCommandLineFormatter>(),
        },
    ),
    (
        "GESCommandLineFormatterClass",
        Layout {
            size: size_of::<GESCommandLineFormatterClass>(),
            alignment: align_of::<GESCommandLineFormatterClass>(),
        },
    ),
    (
        "GESContainer",
        Layout {
            size: size_of::<GESContainer>(),
            alignment: align_of::<GESContainer>(),
        },
    ),
    (
        "GESContainerClass",
        Layout {
            size: size_of::<GESContainerClass>(),
            alignment: align_of::<GESContainerClass>(),
        },
    ),
    (
        "GESEdge",
        Layout {
            size: size_of::<GESEdge>(),
            alignment: align_of::<GESEdge>(),
        },
    ),
    (
        "GESEditMode",
        Layout {
            size: size_of::<GESEditMode>(),
            alignment: align_of::<GESEditMode>(),
        },
    ),
    (
        "GESEffect",
        Layout {
            size: size_of::<GESEffect>(),
            alignment: align_of::<GESEffect>(),
        },
    ),
    (
        "GESEffectAsset",
        Layout {
            size: size_of::<GESEffectAsset>(),
            alignment: align_of::<GESEffectAsset>(),
        },
    ),
    (
        "GESEffectAssetClass",
        Layout {
            size: size_of::<GESEffectAssetClass>(),
            alignment: align_of::<GESEffectAssetClass>(),
        },
    ),
    (
        "GESEffectClass",
        Layout {
            size: size_of::<GESEffectClass>(),
            alignment: align_of::<GESEffectClass>(),
        },
    ),
    (
        "GESEffectClip",
        Layout {
            size: size_of::<GESEffectClip>(),
            alignment: align_of::<GESEffectClip>(),
        },
    ),
    (
        "GESEffectClipClass",
        Layout {
            size: size_of::<GESEffectClipClass>(),
            alignment: align_of::<GESEffectClipClass>(),
        },
    ),
    (
        "GESError",
        Layout {
            size: size_of::<GESError>(),
            alignment: align_of::<GESError>(),
        },
    ),
    (
        "GESExtractableInterface",
        Layout {
            size: size_of::<GESExtractableInterface>(),
            alignment: align_of::<GESExtractableInterface>(),
        },
    ),
    (
        "GESFormatter",
        Layout {
            size: size_of::<GESFormatter>(),
            alignment: align_of::<GESFormatter>(),
        },
    ),
    (
        "GESFormatterClass",
        Layout {
            size: size_of::<GESFormatterClass>(),
            alignment: align_of::<GESFormatterClass>(),
        },
    ),
    (
        "GESFrameNumber",
        Layout {
            size: size_of::<GESFrameNumber>(),
            alignment: align_of::<GESFrameNumber>(),
        },
    ),
    (
        "GESGroup",
        Layout {
            size: size_of::<GESGroup>(),
            alignment: align_of::<GESGroup>(),
        },
    ),
    (
        "GESGroupClass",
        Layout {
            size: size_of::<GESGroupClass>(),
            alignment: align_of::<GESGroupClass>(),
        },
    ),
    (
        "GESImageSource",
        Layout {
            size: size_of::<GESImageSource>(),
            alignment: align_of::<GESImageSource>(),
        },
    ),
    (
        "GESImageSourceClass",
        Layout {
            size: size_of::<GESImageSourceClass>(),
            alignment: align_of::<GESImageSourceClass>(),
        },
    ),
    (
        "GESLayer",
        Layout {
            size: size_of::<GESLayer>(),
            alignment: align_of::<GESLayer>(),
        },
    ),
    (
        "GESLayerClass",
        Layout {
            size: size_of::<GESLayerClass>(),
            alignment: align_of::<GESLayerClass>(),
        },
    ),
    (
        "GESMarkerClass",
        Layout {
            size: size_of::<GESMarkerClass>(),
            alignment: align_of::<GESMarkerClass>(),
        },
    ),
    (
        "GESMarkerFlags",
        Layout {
            size: size_of::<GESMarkerFlags>(),
            alignment: align_of::<GESMarkerFlags>(),
        },
    ),
    (
        "GESMarkerListClass",
        Layout {
            size: size_of::<GESMarkerListClass>(),
            alignment: align_of::<GESMarkerListClass>(),
        },
    ),
    (
        "GESMetaContainerInterface",
        Layout {
            size: size_of::<GESMetaContainerInterface>(),
            alignment: align_of::<GESMetaContainerInterface>(),
        },
    ),
    (
        "GESMetaFlag",
        Layout {
            size: size_of::<GESMetaFlag>(),
            alignment: align_of::<GESMetaFlag>(),
        },
    ),
    (
        "GESMultiFileSource",
        Layout {
            size: size_of::<GESMultiFileSource>(),
            alignment: align_of::<GESMultiFileSource>(),
        },
    ),
    (
        "GESMultiFileSourceClass",
        Layout {
            size: size_of::<GESMultiFileSourceClass>(),
            alignment: align_of::<GESMultiFileSourceClass>(),
        },
    ),
    (
        "GESOperation",
        Layout {
            size: size_of::<GESOperation>(),
            alignment: align_of::<GESOperation>(),
        },
    ),
    (
        "GESOperationClass",
        Layout {
            size: size_of::<GESOperationClass>(),
            alignment: align_of::<GESOperationClass>(),
        },
    ),
    (
        "GESOperationClip",
        Layout {
            size: size_of::<GESOperationClip>(),
            alignment: align_of::<GESOperationClip>(),
        },
    ),
    (
        "GESOperationClipClass",
        Layout {
            size: size_of::<GESOperationClipClass>(),
            alignment: align_of::<GESOperationClipClass>(),
        },
    ),
    (
        "GESOverlayClip",
        Layout {
            size: size_of::<GESOverlayClip>(),
            alignment: align_of::<GESOverlayClip>(),
        },
    ),
    (
        "GESOverlayClipClass",
        Layout {
            size: size_of::<GESOverlayClipClass>(),
            alignment: align_of::<GESOverlayClipClass>(),
        },
    ),
    (
        "GESPipeline",
        Layout {
            size: size_of::<GESPipeline>(),
            alignment: align_of::<GESPipeline>(),
        },
    ),
    (
        "GESPipelineClass",
        Layout {
            size: size_of::<GESPipelineClass>(),
            alignment: align_of::<GESPipelineClass>(),
        },
    ),
    (
        "GESPipelineFlags",
        Layout {
            size: size_of::<GESPipelineFlags>(),
            alignment: align_of::<GESPipelineFlags>(),
        },
    ),
    (
        "GESPitiviFormatter",
        Layout {
            size: size_of::<GESPitiviFormatter>(),
            alignment: align_of::<GESPitiviFormatter>(),
        },
    ),
    (
        "GESPitiviFormatterClass",
        Layout {
            size: size_of::<GESPitiviFormatterClass>(),
            alignment: align_of::<GESPitiviFormatterClass>(),
        },
    ),
    (
        "GESProject",
        Layout {
            size: size_of::<GESProject>(),
            alignment: align_of::<GESProject>(),
        },
    ),
    (
        "GESProjectClass",
        Layout {
            size: size_of::<GESProjectClass>(),
            alignment: align_of::<GESProjectClass>(),
        },
    ),
    (
        "GESSource",
        Layout {
            size: size_of::<GESSource>(),
            alignment: align_of::<GESSource>(),
        },
    ),
    (
        "GESSourceClass",
        Layout {
            size: size_of::<GESSourceClass>(),
            alignment: align_of::<GESSourceClass>(),
        },
    ),
    (
        "GESSourceClip",
        Layout {
            size: size_of::<GESSourceClip>(),
            alignment: align_of::<GESSourceClip>(),
        },
    ),
    (
        "GESSourceClipAsset",
        Layout {
            size: size_of::<GESSourceClipAsset>(),
            alignment: align_of::<GESSourceClipAsset>(),
        },
    ),
    (
        "GESSourceClipAssetClass",
        Layout {
            size: size_of::<GESSourceClipAssetClass>(),
            alignment: align_of::<GESSourceClipAssetClass>(),
        },
    ),
    (
        "GESSourceClipClass",
        Layout {
            size: size_of::<GESSourceClipClass>(),
            alignment: align_of::<GESSourceClipClass>(),
        },
    ),
    (
        "GESTestClip",
        Layout {
            size: size_of::<GESTestClip>(),
            alignment: align_of::<GESTestClip>(),
        },
    ),
    (
        "GESTestClipClass",
        Layout {
            size: size_of::<GESTestClipClass>(),
            alignment: align_of::<GESTestClipClass>(),
        },
    ),
    (
        "GESTextHAlign",
        Layout {
            size: size_of::<GESTextHAlign>(),
            alignment: align_of::<GESTextHAlign>(),
        },
    ),
    (
        "GESTextOverlay",
        Layout {
            size: size_of::<GESTextOverlay>(),
            alignment: align_of::<GESTextOverlay>(),
        },
    ),
    (
        "GESTextOverlayClass",
        Layout {
            size: size_of::<GESTextOverlayClass>(),
            alignment: align_of::<GESTextOverlayClass>(),
        },
    ),
    (
        "GESTextOverlayClip",
        Layout {
            size: size_of::<GESTextOverlayClip>(),
            alignment: align_of::<GESTextOverlayClip>(),
        },
    ),
    (
        "GESTextOverlayClipClass",
        Layout {
            size: size_of::<GESTextOverlayClipClass>(),
            alignment: align_of::<GESTextOverlayClipClass>(),
        },
    ),
    (
        "GESTextVAlign",
        Layout {
            size: size_of::<GESTextVAlign>(),
            alignment: align_of::<GESTextVAlign>(),
        },
    ),
    (
        "GESTimeline",
        Layout {
            size: size_of::<GESTimeline>(),
            alignment: align_of::<GESTimeline>(),
        },
    ),
    (
        "GESTimelineClass",
        Layout {
            size: size_of::<GESTimelineClass>(),
            alignment: align_of::<GESTimelineClass>(),
        },
    ),
    (
        "GESTimelineElement",
        Layout {
            size: size_of::<GESTimelineElement>(),
            alignment: align_of::<GESTimelineElement>(),
        },
    ),
    (
        "GESTimelineElementClass",
        Layout {
            size: size_of::<GESTimelineElementClass>(),
            alignment: align_of::<GESTimelineElementClass>(),
        },
    ),
    (
        "GESTitleClip",
        Layout {
            size: size_of::<GESTitleClip>(),
            alignment: align_of::<GESTitleClip>(),
        },
    ),
    (
        "GESTitleClipClass",
        Layout {
            size: size_of::<GESTitleClipClass>(),
            alignment: align_of::<GESTitleClipClass>(),
        },
    ),
    (
        "GESTitleSource",
        Layout {
            size: size_of::<GESTitleSource>(),
            alignment: align_of::<GESTitleSource>(),
        },
    ),
    (
        "GESTitleSourceClass",
        Layout {
            size: size_of::<GESTitleSourceClass>(),
            alignment: align_of::<GESTitleSourceClass>(),
        },
    ),
    (
        "GESTrack",
        Layout {
            size: size_of::<GESTrack>(),
            alignment: align_of::<GESTrack>(),
        },
    ),
    (
        "GESTrackClass",
        Layout {
            size: size_of::<GESTrackClass>(),
            alignment: align_of::<GESTrackClass>(),
        },
    ),
    (
        "GESTrackElement",
        Layout {
            size: size_of::<GESTrackElement>(),
            alignment: align_of::<GESTrackElement>(),
        },
    ),
    (
        "GESTrackElementAsset",
        Layout {
            size: size_of::<GESTrackElementAsset>(),
            alignment: align_of::<GESTrackElementAsset>(),
        },
    ),
    (
        "GESTrackElementAssetClass",
        Layout {
            size: size_of::<GESTrackElementAssetClass>(),
            alignment: align_of::<GESTrackElementAssetClass>(),
        },
    ),
    (
        "GESTrackElementClass",
        Layout {
            size: size_of::<GESTrackElementClass>(),
            alignment: align_of::<GESTrackElementClass>(),
        },
    ),
    (
        "GESTrackType",
        Layout {
            size: size_of::<GESTrackType>(),
            alignment: align_of::<GESTrackType>(),
        },
    ),
    (
        "GESTransition",
        Layout {
            size: size_of::<GESTransition>(),
            alignment: align_of::<GESTransition>(),
        },
    ),
    (
        "GESTransitionClass",
        Layout {
            size: size_of::<GESTransitionClass>(),
            alignment: align_of::<GESTransitionClass>(),
        },
    ),
    (
        "GESTransitionClip",
        Layout {
            size: size_of::<GESTransitionClip>(),
            alignment: align_of::<GESTransitionClip>(),
        },
    ),
    (
        "GESTransitionClipClass",
        Layout {
            size: size_of::<GESTransitionClipClass>(),
            alignment: align_of::<GESTransitionClipClass>(),
        },
    ),
    (
        "GESUriClip",
        Layout {
            size: size_of::<GESUriClip>(),
            alignment: align_of::<GESUriClip>(),
        },
    ),
    (
        "GESUriClipAsset",
        Layout {
            size: size_of::<GESUriClipAsset>(),
            alignment: align_of::<GESUriClipAsset>(),
        },
    ),
    (
        "GESUriClipAssetClass",
        Layout {
            size: size_of::<GESUriClipAssetClass>(),
            alignment: align_of::<GESUriClipAssetClass>(),
        },
    ),
    (
        "GESUriClipClass",
        Layout {
            size: size_of::<GESUriClipClass>(),
            alignment: align_of::<GESUriClipClass>(),
        },
    ),
    (
        "GESUriSourceAsset",
        Layout {
            size: size_of::<GESUriSourceAsset>(),
            alignment: align_of::<GESUriSourceAsset>(),
        },
    ),
    (
        "GESUriSourceAssetClass",
        Layout {
            size: size_of::<GESUriSourceAssetClass>(),
            alignment: align_of::<GESUriSourceAssetClass>(),
        },
    ),
    (
        "GESVideoSource",
        Layout {
            size: size_of::<GESVideoSource>(),
            alignment: align_of::<GESVideoSource>(),
        },
    ),
    (
        "GESVideoSourceClass",
        Layout {
            size: size_of::<GESVideoSourceClass>(),
            alignment: align_of::<GESVideoSourceClass>(),
        },
    ),
    (
        "GESVideoStandardTransitionType",
        Layout {
            size: size_of::<GESVideoStandardTransitionType>(),
            alignment: align_of::<GESVideoStandardTransitionType>(),
        },
    ),
    (
        "GESVideoTestPattern",
        Layout {
            size: size_of::<GESVideoTestPattern>(),
            alignment: align_of::<GESVideoTestPattern>(),
        },
    ),
    (
        "GESVideoTestSource",
        Layout {
            size: size_of::<GESVideoTestSource>(),
            alignment: align_of::<GESVideoTestSource>(),
        },
    ),
    (
        "GESVideoTestSourceClass",
        Layout {
            size: size_of::<GESVideoTestSourceClass>(),
            alignment: align_of::<GESVideoTestSourceClass>(),
        },
    ),
    (
        "GESVideoTrack",
        Layout {
            size: size_of::<GESVideoTrack>(),
            alignment: align_of::<GESVideoTrack>(),
        },
    ),
    (
        "GESVideoTrackClass",
        Layout {
            size: size_of::<GESVideoTrackClass>(),
            alignment: align_of::<GESVideoTrackClass>(),
        },
    ),
    (
        "GESVideoTransition",
        Layout {
            size: size_of::<GESVideoTransition>(),
            alignment: align_of::<GESVideoTransition>(),
        },
    ),
    (
        "GESVideoTransitionClass",
        Layout {
            size: size_of::<GESVideoTransitionClass>(),
            alignment: align_of::<GESVideoTransitionClass>(),
        },
    ),
    (
        "GESVideoUriSource",
        Layout {
            size: size_of::<GESVideoUriSource>(),
            alignment: align_of::<GESVideoUriSource>(),
        },
    ),
    (
        "GESVideoUriSourceClass",
        Layout {
            size: size_of::<GESVideoUriSourceClass>(),
            alignment: align_of::<GESVideoUriSourceClass>(),
        },
    ),
    (
        "GESXmlFormatter",
        Layout {
            size: size_of::<GESXmlFormatter>(),
            alignment: align_of::<GESXmlFormatter>(),
        },
    ),
    (
        "GESXmlFormatterClass",
        Layout {
            size: size_of::<GESXmlFormatterClass>(),
            alignment: align_of::<GESXmlFormatterClass>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) GES_ASSET_LOADING_ASYNC", "1"),
    ("(gint) GES_ASSET_LOADING_ERROR", "0"),
    ("(gint) GES_ASSET_LOADING_OK", "2"),
    ("(gint) GES_CHILDREN_IGNORE_NOTIFIES", "1"),
    ("(gint) GES_CHILDREN_LAST", "4"),
    ("(gint) GES_CHILDREN_UPDATE", "0"),
    ("(gint) GES_CHILDREN_UPDATE_ALL_VALUES", "3"),
    ("(gint) GES_CHILDREN_UPDATE_OFFSETS", "2"),
    ("(gint) GES_EDGE_END", "1"),
    ("(gint) GES_EDGE_NONE", "2"),
    ("(gint) GES_EDGE_START", "0"),
    ("(gint) GES_EDIT_MODE_NORMAL", "0"),
    ("(gint) GES_EDIT_MODE_RIPPLE", "1"),
    ("(gint) GES_EDIT_MODE_ROLL", "2"),
    ("(gint) GES_EDIT_MODE_SLIDE", "4"),
    ("(gint) GES_EDIT_MODE_TRIM", "3"),
    ("(gint) GES_ERROR_ASSET_LOADING", "1"),
    ("(gint) GES_ERROR_ASSET_WRONG_ID", "0"),
    ("(gint) GES_ERROR_FORMATTER_MALFORMED_INPUT_FILE", "2"),
    ("(gint) GES_ERROR_INVALID_EFFECT_BIN_DESCRIPTION", "8"),
    ("(gint) GES_ERROR_INVALID_FRAME_NUMBER", "3"),
    ("(gint) GES_ERROR_INVALID_OVERLAP_IN_TRACK", "7"),
    ("(gint) GES_ERROR_NEGATIVE_LAYER", "4"),
    ("(gint) GES_ERROR_NEGATIVE_TIME", "5"),
    ("(gint) GES_ERROR_NOT_ENOUGH_INTERNAL_CONTENT", "6"),
    ("GES_FRAME_NUMBER_NONE", "9223372036854775807"),
    ("(guint) GES_MARKER_FLAG_NONE", "0"),
    ("(guint) GES_MARKER_FLAG_SNAPPABLE", "1"),
    ("GES_META_DESCRIPTION", "description"),
    ("GES_META_FORMATTER_EXTENSION", "extension"),
    ("GES_META_FORMATTER_MIMETYPE", "mimetype"),
    ("GES_META_FORMATTER_NAME", "name"),
    ("GES_META_FORMATTER_RANK", "rank"),
    ("GES_META_FORMATTER_VERSION", "version"),
    ("GES_META_FORMAT_VERSION", "format-version"),
    ("GES_META_MARKER_COLOR", "marker-color"),
    ("(guint) GES_META_READABLE", "1"),
    ("(guint) GES_META_READ_WRITE", "3"),
    ("GES_META_VOLUME", "volume"),
    ("GES_META_VOLUME_DEFAULT", "1.000000"),
    ("(guint) GES_META_WRITABLE", "2"),
    ("GES_MULTI_FILE_URI_PREFIX", "multifile://"),
    ("GES_PADDING", "4"),
    ("GES_PADDING_LARGE", "20"),
    ("(guint) GES_PIPELINE_MODE_PREVIEW", "3"),
    ("(guint) GES_PIPELINE_MODE_PREVIEW_AUDIO", "1"),
    ("(guint) GES_PIPELINE_MODE_PREVIEW_VIDEO", "2"),
    ("(guint) GES_PIPELINE_MODE_RENDER", "4"),
    ("(guint) GES_PIPELINE_MODE_SMART_RENDER", "8"),
    ("(gint) GES_TEXT_HALIGN_ABSOLUTE", "5"),
    ("(gint) GES_TEXT_HALIGN_CENTER", "1"),
    ("(gint) GES_TEXT_HALIGN_LEFT", "0"),
    ("(gint) GES_TEXT_HALIGN_POSITION", "4"),
    ("(gint) GES_TEXT_HALIGN_RIGHT", "2"),
    ("(gint) GES_TEXT_VALIGN_ABSOLUTE", "5"),
    ("(gint) GES_TEXT_VALIGN_BASELINE", "0"),
    ("(gint) GES_TEXT_VALIGN_BOTTOM", "1"),
    ("(gint) GES_TEXT_VALIGN_CENTER", "4"),
    ("(gint) GES_TEXT_VALIGN_POSITION", "3"),
    ("(gint) GES_TEXT_VALIGN_TOP", "2"),
    ("GES_TIMELINE_ELEMENT_NO_LAYER_PRIORITY", "4294967295"),
    ("(guint) GES_TRACK_TYPE_AUDIO", "2"),
    ("(guint) GES_TRACK_TYPE_CUSTOM", "16"),
    ("(guint) GES_TRACK_TYPE_TEXT", "8"),
    ("(guint) GES_TRACK_TYPE_UNKNOWN", "1"),
    ("(guint) GES_TRACK_TYPE_VIDEO", "4"),
    ("GES_VERSION_MAJOR", "1"),
    ("GES_VERSION_MICRO", "0"),
    ("GES_VERSION_MINOR", "20"),
    ("GES_VERSION_NANO", "0"),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNDOOR_DBL",
        "45",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNDOOR_DTL",
        "46",
    ),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNDOOR_H", "22"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNDOOR_V", "21"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNVEE_D", "65"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNVEE_L", "66"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNVEE_R", "68"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BARNVEE_U", "67"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BAR_WIPE_LR", "1"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BAR_WIPE_TB", "2"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOWTIE_H", "44"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOWTIE_V", "43"),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_BC",
        "25",
    ),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_BL", "6"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_BR", "5"),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_LC",
        "26",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_RC",
        "24",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_TC",
        "23",
    ),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_TL", "3"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_BOX_WIPE_TR", "4"),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_CLOCK_CW12",
        "201",
    ),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_CLOCK_CW3", "202"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_CLOCK_CW6", "203"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_CLOCK_CW9", "204"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_CROSSFADE", "512"),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DIAGONAL_TL",
        "41",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DIAGONAL_TR",
        "42",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLEFAN_FIH",
        "236",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLEFAN_FIV",
        "235",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLEFAN_FOH",
        "214",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLEFAN_FOV",
        "213",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLESWEEP_OH",
        "228",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLESWEEP_OV",
        "227",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLESWEEP_PD",
        "226",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLESWEEP_PDBL",
        "246",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLESWEEP_PDTL",
        "245",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_DOUBLESWEEP_PV",
        "225",
    ),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FAN_B", "233"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FAN_CR", "212"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FAN_CT", "211"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FAN_L", "234"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FAN_R", "232"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FAN_T", "231"),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FOUR_BOX_WIPE_CI",
        "7",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_FOUR_BOX_WIPE_CO",
        "8",
    ),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_IRIS_RECT", "101"),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_MISC_DIAGONAL_DBD",
        "47",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_MISC_DIAGONAL_DD",
        "48",
    ),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_NONE", "0"),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_PINWHEEL_FB",
        "207",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_PINWHEEL_TBH",
        "206",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_PINWHEEL_TBV",
        "205",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SALOONDOOR_B",
        "253",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SALOONDOOR_L",
        "252",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SALOONDOOR_R",
        "254",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SALOONDOOR_T",
        "251",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWB",
        "223",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWBL",
        "242",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWBR",
        "243",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWL",
        "224",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWR",
        "222",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWT",
        "221",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWTL",
        "241",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_SINGLESWEEP_CWTR",
        "244",
    ),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_VEE_D", "61"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_VEE_L", "62"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_VEE_R", "64"),
    ("(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_VEE_U", "63"),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_WINDSHIELD_H",
        "264",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_WINDSHIELD_R",
        "261",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_WINDSHIELD_U",
        "262",
    ),
    (
        "(gint) GES_VIDEO_STANDARD_TRANSITION_TYPE_WINDSHIELD_V",
        "263",
    ),
    ("(gint) GES_VIDEO_TEST_CHROMA_ZONE_PLATE", "16"),
    ("(gint) GES_VIDEO_TEST_GAMUT", "15"),
    ("(gint) GES_VIDEO_TEST_PATTERN_BLACK", "2"),
    ("(gint) GES_VIDEO_TEST_PATTERN_BLINK", "12"),
    ("(gint) GES_VIDEO_TEST_PATTERN_BLUE", "6"),
    ("(gint) GES_VIDEO_TEST_PATTERN_CHECKERS1", "7"),
    ("(gint) GES_VIDEO_TEST_PATTERN_CHECKERS2", "8"),
    ("(gint) GES_VIDEO_TEST_PATTERN_CHECKERS4", "9"),
    ("(gint) GES_VIDEO_TEST_PATTERN_CHECKERS8", "10"),
    ("(gint) GES_VIDEO_TEST_PATTERN_CIRCULAR", "11"),
    ("(gint) GES_VIDEO_TEST_PATTERN_GREEN", "5"),
    ("(gint) GES_VIDEO_TEST_PATTERN_RED", "4"),
    ("(gint) GES_VIDEO_TEST_PATTERN_SMPTE", "0"),
    ("(gint) GES_VIDEO_TEST_PATTERN_SMPTE75", "13"),
    ("(gint) GES_VIDEO_TEST_PATTERN_SNOW", "1"),
    ("(gint) GES_VIDEO_TEST_PATTERN_SOLID", "17"),
    ("(gint) GES_VIDEO_TEST_PATTERN_WHITE", "3"),
    ("(gint) GES_VIDEO_TEST_ZONE_PLATE", "14"),
];
