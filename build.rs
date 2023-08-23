use std::path::PathBuf;

/// build library name
#[allow(dead_code)]
const LIB_NAME: &'static str = "quickjs";


/// build library path env(QUICKJS_SOURCE_PATH)
#[allow(dead_code)]
const LIB_SOURCE_PATH_ENV: &'static str = "QUICKJS_SOURCE_PATH";


/// build library extends(QUICKJS_SOURCE_EXTENDS_PATH)
const LIB_EXTENDS_PATH_ENV: &'static str = "QUICKJS_SOURCE_EXTENDS_PATH";

/// default extends path
const LIB_EXTENDS_DEFAULT_PATH: &'static str = "extends";

/// link static library path env(QUICKJS_LIB_PATH)
#[allow(dead_code)]
const LIB_LINK_PATH_ENV: &'static str = "QUICKJS_LIB_PATH";


/// file/path exists
///
/// ```rust
/// let f =  "Cargo.toml";
/// if exists("f") {
///     println("file exists!");
/// }
/// ```
#[allow(dead_code)]
fn exists(f: impl AsRef<std::path::Path>) -> bool {
    return PathBuf::from(f.as_ref()).exists();
}


/// build .c files
const LIB_C_FILES: [&'static str; 5] = [
    "cutils.c",
    "libbf.c",
    "libregexp.c",
    "libunicode.c",
    "quickjs.c",
];

/// import extend .c files
fn extend_file(d: impl AsRef<std::path::Path>, f: impl AsRef<std::path::Path>) -> Result<String, std::io::Error> {
    // filename
    let file = PathBuf::from(f.as_ref());
    let filename = file.file_name().unwrap().to_os_string();
    let filename = filename.into_string().unwrap();

    // directory
    let dir = PathBuf::from(d.as_ref());
    let dir = dir.join(filename);
    if dir.exists() {
        let _ = std::fs::remove_file(&dir);
    }

    // copy
    std::fs::copy(&file, &dir)?;
    Ok(dir.display().to_string())
}


/// create wrapper file
fn wrapper_file(f: impl AsRef<std::path::Path>, ext: impl AsRef<std::path::Path>) -> Result<String, std::io::Error> {
    let filename = "_wrapper.h";

    // clean
    let file = PathBuf::from(f.as_ref());
    let file = file.join(filename);
    if file.exists() {
        let _ = std::fs::remove_file(&file);
    }

    // ext
    let extends = PathBuf::from(ext.as_ref());
    if extends.exists() {
        let wrapper = extends.join(filename);
        if wrapper.exists() {
            if let Ok(_) = std::fs::copy(wrapper,&file){
                return Ok(file.display().to_string());
            };
        }
    }

    // wrapper output
    std::fs::write(&file, "#include \"quickjs.h\"")?;
    Ok(file.display().to_string())
}


/// Linux build
///
/// require:
/// - gcc ( for Ubuntu: apt install gcc )
/// - make ( for Ubuntu: apt install make )
/// - libclang( for Ubuntu: apt install libclang-dev )
///
#[cfg(target_os = "linux")]
fn main() {
    let source_path_str = if let Ok(source) = std::env::var(LIB_SOURCE_PATH_ENV) {
        source
    } else {
        LIB_NAME.into()
    };

    // directory exists ?
    let source_path = std::path::Path::new(&source_path_str);
    if !source_path.exists() {
        panic!("failed by quickjs source path")
    };

    // version exists ?
    let version_file = source_path.join("VERSION");
    if !version_file.exists() {
        panic!("failed by quickjs source version")
    }
    let version = std::fs::read_to_string(version_file)
        .expect("failed by quickjs source version");
    println!("cargo:warning=QUICKJS LIB VERSION:{}", version);

    // link files
    let mut files: Vec<_> = LIB_C_FILES
        .map(|f| source_path.join(f))
        .into_iter()
        .collect();

    // extends files
    let extends_path_str = std::env::var(LIB_EXTENDS_PATH_ENV)
        .unwrap_or(LIB_EXTENDS_DEFAULT_PATH.to_string());
    let extends_path = std::path::Path::new(extends_path_str.as_str());
    if extends_path.exists() {
        if let Ok(dir) = std::fs::read_dir(extends_path) {
            for file in dir {
                if let Ok(f) = file {
                    if let Some(ext) = f.path().extension() {
                        if ext == "c" {
                            if let Ok(out_file) = extend_file(&source_path_str, f.path()) {
                                files.push(PathBuf::from(out_file));
                            };
                        }
                    }
                }
            }
        }
    }


    // compile code
    cc::Build::default()
        .files(files)
        .define("_GUN_SOURCE", None)
        .define("CONFIG_VERSION", format!("\"{}\"", version.trim()).as_str())
        .define("CONFIG_BIGNUM", None)
        .flag_if_supported("-Wchar-subscripts")
        .flag_if_supported("-Wno-array-bounds")
        .flag_if_supported("-Wno-format-truncation")
        .flag_if_supported("-Wno-missing-field-initializers")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wundef")
        .flag_if_supported("-Wuninitialized")
        .flag_if_supported("-Wunused")
        .flag_if_supported("-Wwrite-strings")
        .flag_if_supported("-funsigned-char")
        .flag_if_supported("-Wno-cast-function-type")
        .flag_if_supported("-Wno-implicit-fallthrough")
        .flag_if_supported("-Wno-enum-conversion")
        .opt_level(2)
        .flag_if_supported("-latomic")
        .compile(LIB_NAME);


    // create bindings
    let wrapper_file = wrapper_file(source_path_str, extends_path_str).unwrap();
    let bindings = bindgen::Builder::default()
        .header(wrapper_file)
        .size_t_is_usize(true)
        .generate_inline_functions(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let rs_out_file = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let rs_out_file = rs_out_file.join("bindings.rs");
    bindings.write_to_file(&rs_out_file).expect("Couldn't write bindings.rs!");
    println!("cargo:warning=Build rust file:{:?}", rs_out_file);
    println!("cargo:rustc-link-lib={}", LIB_NAME);
}


#[cfg(all(not(target_os = "linux"), not(target_os = "mac")))]
fn main() {
    panic!("Non Supported Build")
}
