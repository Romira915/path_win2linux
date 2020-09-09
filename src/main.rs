#[macro_use]
extern crate clap;
use clap::{App, Arg};
use log::{error, info, warn};
use std::{
    fs::{self},
    io::{self, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::exit,
};

const DEFAULT_EXTENION: &str = "m3u";

// Get file list of directory
fn get_filepath<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>, io::Error> {
    let entries = fs::read_dir(dir)?
        .filter_map(|res| {
            if let Ok(e) = res {
                let path = e.path();
                if path.is_file() {
                    return Some(path);
                }
            }
            None
        })
        .collect::<Vec<PathBuf>>();

    Ok(entries)
}

fn filter_extension(files: Vec<PathBuf>, argv: &str) -> Vec<PathBuf> {
    files
        .into_iter()
        .filter(|f| {
            if let Some(e) = f.extension() {
                e == argv
            } else {
                false
            }
        })
        .collect()
}

fn path_convert(files: &Vec<PathBuf>) -> Result<(), io::Error> {
    for path in files {
        let f = fs::OpenOptions::new().read(true).open(path)?;
        let mut reader = BufReader::new(f);

        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let buf = buf
            .iter()
            .map(|f| {
                if *f == b'\\' {
                    return b'/';
                }
                *f
            })
            .collect::<Box<[u8]>>();

        let mut f = fs::OpenOptions::new()
            .write(true)
            .append(false)
            .open(path)?;

        f.write_all(&buf)?;
        info!("Wrote {}", path.display());
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("file")
                .short("-f")
                .long("--file")
                .takes_value(true)
                .value_name("FILE")
                .help("Set the file path to convert"),
        )
        .arg(
            Arg::with_name("dir")
            .short("-d")
                .long("--dir")
                .takes_value(true)
                .value_name("DIR")
                .help("Convert the files in the specified directory."),
        )
        .arg(
            Arg::with_name("ext").short("-e").long("--extension").value_name("EXTENSION").takes_value(true).help("Specify the extension of the file to be converted. This is valid only when \"-d\" is specified.\n\
                                                                                                                            Don't include \".\"")
        ).arg(
            Arg::with_name("log").short("-l").long("--log").takes_value(true).value_name("LOGLEVEL").help("Specify the log level to be output.\n\
                                                                                                                       <0> error only\n\
                                                                                                                       <1> include warn\n\
                                                                                                                       <2> include info\n\
                                                                                                                       Default 2")
        );

    let matches = app.get_matches();

    let (level, arg_warn) = if let Ok(log) = matches.value_of("log").unwrap_or("2").parse() {
        (
            match log {
                0 => "error",
                1 => "worn",
                _ => "info",
            },
            None,
        )
    } else {
        (
            "info",
            Some(
                "The value specified by the argument LOG is invalid. May not be a number. Set default value 2.",
            ),
        )
    };

    std::env::set_var("RUST_LOG", level);
    env_logger::init();
    if let Some(w) = arg_warn {
        warn!("{}", w);
    }

    let mut files = Vec::new();

    // Get a file from the directory of argument dir
    if let Some(argv) = matches.value_of("dir") {
        match get_filepath(argv) {
            Ok(mut v) => {
                files.append(&mut v);
                info!("Get file path.");
            }
            Err(e) => error!("{}", e),
        }

        // Extract only the extension of the argument ext
        if let Some(argv) = matches.value_of("ext") {
            files = filter_extension(files, argv);
            if files.len() == 0 {
                error!("Not found file: extension \"{}\".", argv);
                if argv.starts_with(".") {
                    error!("Don't include \".\"");
                }
                std::process::exit(-1);
            } else {
                info!("Extract file: \"{}\".", argv);
            }
        }
    } else if let Some(_) = matches.value_of("ext") {
        error!("The argument DIR is not specified. This is required if you specify the EXTENSION argument.");
    }

    if let Some(argv) = matches.value_of("file") {
        match fs::metadata(argv) {
            Ok(metadata) => {
                if metadata.is_file() {
                    files.push(PathBuf::from(argv));
                }
            }
            Err(e) => {
                error!("{}", e);
                exit(-2);
            }
        }
    }

    // Default behavior
    if matches.args.len() == 0 {
        match get_filepath(".") {
            Ok(mut v) => {
                files.append(&mut v);
                info!("Get file path.");
            }
            Err(e) => error!("{}", e),
        }
        files = filter_extension(files, DEFAULT_EXTENION);
    }

    path_convert(&files)?;

    Ok(())
}
