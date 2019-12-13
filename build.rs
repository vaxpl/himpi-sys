use bindgen;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

type DynError = Box<dyn std::error::Error>;

#[derive(Clone, Default)]
struct MyError(String);

impl MyError {
    pub fn new<S: AsRef<str>>(msg: S) -> Self {
        Self {
            0: String::from(msg.as_ref()),
        }
    }
}

impl std::fmt::Debug for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MyError {{ {} }}", self.0)
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for MyError {}

macro_rules! MyErr {
    ($msg: expr) => {
        Err(Box::new(MyError::new($msg)))
    };
}

#[derive(Copy, Clone, Debug, Default)]
struct MyParseCallbacks;

impl bindgen::callbacks::ParseCallbacks for MyParseCallbacks {
    /// Allows to rename an item, replacing `original_item_name`.
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        match original_item_name {
            "hiVB_CONF_S_hiVB_CPOOL_S" => {
                return Some(String::from("VB_CPOOL_S"));
            }
            _ => {}
        }
        let re = Regex::new(r"^(hi)([^a-z]+)$").unwrap();
        for cap in re.captures_iter(original_item_name) {
            return Some(format!("{}", &cap[2]));
        }
        let re = Regex::new(r"^(hi)([^a-z]+)__bindgen_ty_(\d)$").unwrap();
        for cap in re.captures_iter(original_item_name) {
            return Some(format!("{}_U{}", &cap[2], &cap[3]));
        }
        None
    }
}

#[cfg(feature = "static-link")]
macro_rules! linklib {
    ($x:expr) => {
        println!("cargo:rustc-link-lib=static={}", $x);
    };
}

#[cfg(not(feature = "static-link"))]
macro_rules! linklib {
    ($x:expr) => {
        println!("cargo:rustc-link-lib=dylib={}", $x);
    };
}

fn detect_path(base: &str, dir: &str) -> Result<PathBuf, DynError> {
    let mut base_path = Path::new(&base);
    for _a in 0..9 {
        let np = base_path.join(dir);
        let path = Path::new(&np);
        if path.exists() {
            return Ok(path.to_path_buf());
        }
        match base_path.parent() {
            Some(v) => base_path = v,
            None => break,
        }
    }
    MyErr!(format!("Dir `{}` does not detected!", dir))
}

fn detect_mpp_path(mpp_dir: &str) -> Result<PathBuf, DynError> {
    let base1 = env::var("CARGO_MANIFEST_DIR").unwrap();
    let base2 = env::var("OUT_DIR").unwrap();
    detect_path(&base1, mpp_dir)
        .or_else(|_| detect_path(&base2, mpp_dir))
        .or(MyErr!(format!(
            "The `MPP_DIR={}` does not detected!",
            mpp_dir
        )))
}

fn setup_envir() -> Result<(), DynError> {
    match env::var("TARGET") {
        Ok(val) => {
            if val == "x86_64-unknown-linux-gnu" {
                return MyErr!("Target not supported!");
            }
        }
        _ => {}
    }

    match env::var("MPP_DIR") {
        Err(_) => {
            #[cfg(any(
                feature = "hi3516ev200",
                feature = "hi3516ev300",
                feature = "hi3518ev200",
                feature = "hi3518ev300"
            ))]
            env::set_var(
                "MPP_DIR",
                detect_mpp_path("vendor/mpp-lib-Hi3516EV200_V1.0.1.0").unwrap(),
            );

            #[cfg(feature = "hi3531v100")]
            env::set_var(
                "MPP_DIR",
                detect_mpp_path("vendor/mpp-lib-Hi3531V100_V1.0.D.0").unwrap(),
            );

            #[cfg(feature = "hi3559av100")]
            env::set_var(
                "MPP_DIR",
                detect_mpp_path("vendor/mpp-lib-Hi3559AV100_V2.0.2.0").unwrap(),
            );
        }
        _ => {}
    };

    match env::var("SYS_INCLUDE") {
        Err(_) => {
            #[cfg(any(
                feature = "hi3516ev200",
                feature = "hi3516ev300",
                feature = "hi3518ev200",
                feature = "hi3518ev300"
            ))]
            env::set_var(
                "SYS_INCLUDE",
                "/opt/hisi-linux/x86-arm/arm-himix100-linux/target/usr/include",
            );

            #[cfg(feature = "hi3531v100")]
            env::set_var(
                "SYS_INCLUDE",
                "/opt/hisi-linux-nptl/arm-hisiv100-linux/target/usr/include",
            );

            #[cfg(feature = "hi3559av100")]
            env::set_var(
                "SYS_INCLUDE",
                "/opt/hisi-linux/x86-arm/aarch64-himix100-linux/aarch64-linux-gnu/sys-include",
            );
        }
        _ => {}
    }

    match env::var("SYS_LIBDIR") {
        Err(_) => {
            #[cfg(any(
                feature = "hi3516ev200",
                feature = "hi3516ev300",
                feature = "hi3518ev200",
                feature = "hi3518ev300"
            ))]
            env::set_var(
                "SYS_LIBDIR",
                "/opt/hisi-linux/x86-arm/arm-himix100-linux/target/usr/lib",
            );

            #[cfg(feature = "hi3531v100")]
            env::set_var(
                "SYS_LIBDIR",
                "/opt/hisi-linux-nptl/arm-hisiv100-linux/target/usr/lib",
            );

            #[cfg(feature = "hi3559av100")]
            env::set_var(
                "SYS_LIBDIR",
                "/opt/hisi-linux/x86-arm/aarch64-himix100-linux/target/usr/lib",
            );
        }
        _ => {}
    }

    Ok(())
}

fn main() -> Result<(), DynError> {
    if cfg!(not(any(
        feature = "hi3516ev200",
        feature = "hi3516ev300",
        feature = "hi3518ev200",
        feature = "hi3518ev300",
        feature = "hi3519av100",
        feature = "hi3531v100",
        feature = "hi3559av100",
    ))) {
        return MyErr!("The target board does not specified!");
    }

    println!("cargo:rerun-if-env-changed=MPP_DIR");
    println!("cargo:rerun-if-env-changed=SYS_INCLUDE");
    println!("cargo:rerun-if-env-changed=SYS_LIBDIR");
    println!("cargo:rerun-if-changed=build.rs");

    setup_envir()?;

    let mpp_dir = env::var("MPP_DIR").unwrap();
    if Path::new(&mpp_dir).exists() == false {
        return MyErr!(format!("The `MPP_DIR={}` does not exists", mpp_dir));
    }

    let wrapper_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("mpp-wrapper.h");
    let wrapper_path = wrapper_path.to_str().unwrap();
    let mut wrapper = File::create(wrapper_path).unwrap();
    writeln!(wrapper, "#include <stddef.h>")?;
    writeln!(wrapper, "#include <stdint.h>")?;

    if cfg!(feature = "mpi-sys") {
        writeln!(wrapper, "#include <hi_common.h>")?;
        writeln!(wrapper, "#include <mpi_sys.h>")?;
        linklib!("mpi");
        if cfg!(feature = "hi3531v100") {
            linklib!("jpeg");
            linklib!("VoiceEngine");
            linklib!("aec");
            linklib!("anr");
            linklib!("resampler");
            linklib!("vqev2");
        } else {
            linklib!("VoiceEngine");
            linklib!("dnvqe");
            linklib!("upvqe");
            linklib!("securec");
        }
    }

    if cfg!(feature = "mpi-vb") {
        writeln!(wrapper, "#include <mpi_vb.h>")?;
    }

    if cfg!(feature = "mpi-audio") {
        if cfg!(feature = "hi3531v100") {
            writeln!(wrapper, "#include <mpi_adec.h>")?;
            writeln!(wrapper, "#include <mpi_aenc.h>")?;
            writeln!(wrapper, "#include <mpi_ai.h>")?;
            writeln!(wrapper, "#include <mpi_ao.h>")?;
        } else {
            writeln!(wrapper, "#include <mpi_audio.h>")?;
            linklib!("aacdec");
            linklib!("aacenc");
        }
    }

    if cfg!(feature = "mpi-hdmi") {
        writeln!(wrapper, "#include <mpi_hdmi.h>")?;
        //#[cfg(all(feature = "static", feature = "hi3531v100"))]
        linklib!("hdmi");
    }

    if cfg!(feature = "mpi-vdec") {
        writeln!(wrapper, "#include <mpi_venc.h>")?;
    }

    if cfg!(feature = "mpi-venc") {
        writeln!(wrapper, "#include <mpi_venc.h>")?;
    }

    if cfg!(feature = "mpi-vi") {
        writeln!(wrapper, "#include <mpi_vi.h>")?;
    }

    if cfg!(feature = "mpi-vo") {
        writeln!(wrapper, "#include <mpi_vo.h>")?;
    }

    if cfg!(feature = "mpi-vpss") {
        writeln!(wrapper, "#include <mpi_vpss.h>")?;
    }

    if cfg!(any(
        feature = "hi3516ev200",
        feature = "hi3516ev300",
        feature = "hi3518ev200",
        feature = "hi3518ev300",
        feature = "hi3531v100"
    )) {
        #[cfg(not(feature = "static-link"))]
        linklib!("pthread");
    }

    println!(
        "cargo:rustc-link-search=native={}/lib",
        env::var("MPP_DIR").unwrap()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        env::var("SYS_LIBDIR").unwrap()
    );

    let bindings = bindgen::Builder::default()
        .header(wrapper_path)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .union_fields_prefix("un")
        .derive_debug(true)
        .impl_debug(false)
        .derive_default(true)
        .derive_partialeq(true)
        .derive_eq(true)
        .impl_partialeq(true)
        .blacklist_type("^VPSS_NRX_V1_S|^tV56a.*$")
        .whitelist_function("^HI_.*")
        .whitelist_type("^HI_.*$")
        .whitelist_type("*._[ES]$")
        .whitelist_var("^HI_.*")
        .whitelist_var("^BIND.*|^VB_.*|^POOL_.*")
        .whitelist_var("^(VDEC|VENC|VI|VO|VPSS)_.*")
        .whitelist_var("^(NR|PRORES)_.*")
        .use_core()
        .clang_arg(format!("-I{}/include", env::var("MPP_DIR").unwrap()))
        .clang_arg(format!("-I{}", env::var("SYS_INCLUDE").unwrap()))
        .parse_callbacks(Box::new(MyParseCallbacks::default()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
