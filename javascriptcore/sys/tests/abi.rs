// Generated by gir (https://github.com/gtk-rs/gir @ b3147f2b6043)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 7fa401e3ee5d)
// from webkit2gtk-gir-files
// DO NOT EDIT

use javascriptcore5_rs_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["javascriptcoregtk-5.0"];

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
        let (name, value) = l.split_once(';').expect("Missing ';' separator");
        c_constants.push((name.to_owned(), value.to_owned()));
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
        let (name, value) = l.split_once(';').expect("Missing first ';' separator");
        let (size, alignment) = value.split_once(';').expect("Missing second ';' separator");
        let size = size.parse().expect("Failed to parse size");
        let alignment = alignment.parse().expect("Failed to parse alignment");
        c_layouts.push((name.to_owned(), Layout { size, alignment }));
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
        "JSCCheckSyntaxMode",
        Layout {
            size: size_of::<JSCCheckSyntaxMode>(),
            alignment: align_of::<JSCCheckSyntaxMode>(),
        },
    ),
    (
        "JSCCheckSyntaxResult",
        Layout {
            size: size_of::<JSCCheckSyntaxResult>(),
            alignment: align_of::<JSCCheckSyntaxResult>(),
        },
    ),
    (
        "JSCClassVTable",
        Layout {
            size: size_of::<JSCClassVTable>(),
            alignment: align_of::<JSCClassVTable>(),
        },
    ),
    (
        "JSCContext",
        Layout {
            size: size_of::<JSCContext>(),
            alignment: align_of::<JSCContext>(),
        },
    ),
    (
        "JSCContextClass",
        Layout {
            size: size_of::<JSCContextClass>(),
            alignment: align_of::<JSCContextClass>(),
        },
    ),
    (
        "JSCException",
        Layout {
            size: size_of::<JSCException>(),
            alignment: align_of::<JSCException>(),
        },
    ),
    (
        "JSCExceptionClass",
        Layout {
            size: size_of::<JSCExceptionClass>(),
            alignment: align_of::<JSCExceptionClass>(),
        },
    ),
    (
        "JSCOptionType",
        Layout {
            size: size_of::<JSCOptionType>(),
            alignment: align_of::<JSCOptionType>(),
        },
    ),
    (
        "JSCValue",
        Layout {
            size: size_of::<JSCValue>(),
            alignment: align_of::<JSCValue>(),
        },
    ),
    (
        "JSCValueClass",
        Layout {
            size: size_of::<JSCValueClass>(),
            alignment: align_of::<JSCValueClass>(),
        },
    ),
    (
        "JSCValuePropertyFlags",
        Layout {
            size: size_of::<JSCValuePropertyFlags>(),
            alignment: align_of::<JSCValuePropertyFlags>(),
        },
    ),
    (
        "JSCVirtualMachine",
        Layout {
            size: size_of::<JSCVirtualMachine>(),
            alignment: align_of::<JSCVirtualMachine>(),
        },
    ),
    (
        "JSCVirtualMachineClass",
        Layout {
            size: size_of::<JSCVirtualMachineClass>(),
            alignment: align_of::<JSCVirtualMachineClass>(),
        },
    ),
    (
        "JSCWeakValue",
        Layout {
            size: size_of::<JSCWeakValue>(),
            alignment: align_of::<JSCWeakValue>(),
        },
    ),
    (
        "JSCWeakValueClass",
        Layout {
            size: size_of::<JSCWeakValueClass>(),
            alignment: align_of::<JSCWeakValueClass>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) JSC_CHECK_SYNTAX_MODE_MODULE", "1"),
    ("(gint) JSC_CHECK_SYNTAX_MODE_SCRIPT", "0"),
    ("(gint) JSC_CHECK_SYNTAX_RESULT_IRRECOVERABLE_ERROR", "2"),
    ("(gint) JSC_CHECK_SYNTAX_RESULT_OUT_OF_MEMORY_ERROR", "4"),
    ("(gint) JSC_CHECK_SYNTAX_RESULT_RECOVERABLE_ERROR", "1"),
    ("(gint) JSC_CHECK_SYNTAX_RESULT_STACK_OVERFLOW_ERROR", "5"),
    ("(gint) JSC_CHECK_SYNTAX_RESULT_SUCCESS", "0"),
    (
        "(gint) JSC_CHECK_SYNTAX_RESULT_UNTERMINATED_LITERAL_ERROR",
        "3",
    ),
    ("JSC_MAJOR_VERSION", "2"),
    ("JSC_MICRO_VERSION", "1"),
    ("JSC_MINOR_VERSION", "34"),
    ("JSC_OPTIONS_USE_DFG", "useDFGJIT"),
    ("JSC_OPTIONS_USE_FTL", "useFTLJIT"),
    ("JSC_OPTIONS_USE_JIT", "useJIT"),
    ("JSC_OPTIONS_USE_LLINT", "useLLInt"),
    ("(gint) JSC_OPTION_BOOLEAN", "0"),
    ("(gint) JSC_OPTION_DOUBLE", "4"),
    ("(gint) JSC_OPTION_INT", "1"),
    ("(gint) JSC_OPTION_RANGE_STRING", "6"),
    ("(gint) JSC_OPTION_SIZE", "3"),
    ("(gint) JSC_OPTION_STRING", "5"),
    ("(gint) JSC_OPTION_UINT", "2"),
    ("(guint) JSC_VALUE_PROPERTY_CONFIGURABLE", "1"),
    ("(guint) JSC_VALUE_PROPERTY_ENUMERABLE", "2"),
    ("(guint) JSC_VALUE_PROPERTY_WRITABLE", "4"),
];
