use zip::ZipArchive;
use flate2::read::GzDecoder;
use flate2::GzHeader;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::process;
use std::io;
use std::env;


/// Information to be included in finished
/// zip, gz, xz, tar report
/// defined here.



struct InfoZip {
    name: PathBuf,
    size_when_uncompressed: u64,
    compressed_size: u64,
    crc_32: u32,
    type_of_dir: bool,
    encrypted: bool,
}

struct InfoGz {
    name: String,
    extra: String,
    comment: String,
    mtime: u32,
    os: u8,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        process::exit(1);
    }
    let dir = Path::new("queue/");
    let target: PathBuf = dir.join(args[1].as_str());

    //let entries: Vec<EntryZip> = handle_zip(&path)?;
    handle_gz(&target)?;
    Ok(())
}

/// Extracts the zip file and wraps the result as InfoZip
/// and hands it to the JSON serializer for export to the host.
/// all variables declared are included in the exported report
fn handle_zip(file_path: &Path) -> io::Result<Vec<InfoZip>> {
    let f = File::open(file_path)?;
    let mut zip_file = ZipArchive::new(f)?;
    let mut entries: Vec<InfoZip> = Vec::new();

    for i in 0..zip_file.len() {
        let file = zip_file.by_index(i)?;

        let err = io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unsafe path in entry: {:?}", file.name()),
        );
        
        let name = file.enclosed_name().ok_or(err)?.to_path_buf();
        let size_when_uncompressed: u64 = file.size();
        let compressed_size: u64 = file.compressed_size();
        let crc_32: u32 = file.crc32();
        let type_of_dir: bool = file.is_dir();
        let encrypted = file.encrypted();

        entries.push(InfoZip {
            name,
            size_when_uncompressed,
            compressed_size,
            crc_32,
            type_of_dir,
            encrypted,
        });
    }
    Ok(entries)
}

/// Decodes the gz file, wraps the result as a GzInfo,
/// and hands it to the JSON serializer for export to the host.
/// Handles all options received from the gz file header,
/// prints to stdout.
fn handle_gz(file_path: &Path) -> io::Result<InfoGz> {
    let f = File::open(file_path)?;
    let gz = GzDecoder::new(&f);

    let name = gz.header()
        .and_then(|h| h.filename())
        .map(|b| String::from_utf8_lossy(name).into_owned())
        .unwrap_or_default();
    let extra = gz.header()
        .and_then(|h| h.extra())
        .map(|b| String::from_utf8_lossy(extra).into_owned())
        .unwrap_or_default();

    let comment = gz.header()
        .and_then(|h| h.comment())
        .map(|b| String::from_utf8_lossy(comment).into_owned())
        .unwrap_or_default();
    let os = gz.header().map(|h| h.operating_system()).unwrap_or(255));
    let mtime = gz.header().map(|h| h.mtime()).unwrap_or(0));


    Ok(InfoGz {
        name,
        extra,
        comment,
        os,
        mtime,
    })
}

/// Returns all file paths in the given directory.
/*fn get_paths(dir: &Path) -> io::Result<Vec<PathBuf>> {
}   
