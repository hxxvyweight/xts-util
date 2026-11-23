use zip::ZipArchive;
use flate2::read::GzDecoder;
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::env;

const TARGET_FILE: &str = "6mb.gz";

struct EntryZip {
    name: PathBuf,
    size_when_uncompressed: u64,
    compressed_size: u64,
    crc_32: u32,
    type_of_dir: bool,
    encrypted: bool,
}



fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <query> <file>", args[0]);
        process::exit(1);
    }
    


    let path = Path::new("queue/");
    let target_dir = path.join(TARGET_FILE);
    println!("{:?}", target_dir);

    //let entries: Vec<EntryZip> = handle_zip(&path)?;
    handle_gz(&target_dir)?;
    Ok(())
}


fn handle_zip(file_path: &Path) -> io::Result<Vec<EntryZip>> {

    let f = fs::File::open(file_path)?;
    let mut zip_file = ZipArchive::new(f)?;
    let mut entries: Vec<EntryZip> = Vec::new();
   
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

        entries.push(EntryZip {
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

fn handle_gz(file_path: &Path) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    let gz = GzDecoder::new(&file);

    if let Some(h) = gz.header() {
        match h.filename() {
            Some(bytes) => {
                if let Ok(s) = String::from_utf8(bytes.to_vec()) {
                    println!("Filename: {:?}", s);
                }
            } 
            None => println!("couldn't get valid filename"),
        }
        println!("mtime: {}", h.mtime());
    }
    Ok(())

}

