use std::{fs::{self, File}, path::Path};
use std::io::{SeekFrom};
use binary_stream::{BinaryReader, BinaryWriter, Options};
use lz4_flex::block::{decompress};

pub struct MDB1Header {
    pub magic: u32,
    pub file_entry_count: i32,
    pub file_name_count: i32,
    pub data_entry_count: i32,
    pub data_start: u64,
    pub total_size: u64
}

pub struct FileEntry {
    pub compare_bit: i32,
    pub data_id: i32,
    pub left: i32,
    pub right: i32
}

pub struct NameEntry {
    pub extension: String,
    pub name: String
}

pub struct DataEntry {
    pub offset: u64,
    pub size: u64,
    pub comp_size: u64
}

pub fn extract(file_path: &str) ->  Result<(), std::io::Error> {
    let f =  File::open(file_path)?;
    let mut reader = BinaryReader::new(f, Options::default());
    let header = MDB1Header{
        magic: reader.read_u32().unwrap(),
        file_entry_count: reader.read_i32().unwrap(),
        file_name_count: reader.read_i32().unwrap(),
        data_entry_count: reader.read_i32().unwrap(),
        data_start: reader.read_u64().unwrap(),
        total_size: reader.read_u64().unwrap()
    };

    assert_eq!(header.magic, 0x3142444D);
    
    let mut file_entries = Vec::<FileEntry>::new();
    let mut name_entries = Vec::<NameEntry>::new();
    let mut data_entries = Vec::<DataEntry>::new();

    for _i in 0..header.file_entry_count{
        file_entries.push(FileEntry{
            compare_bit: reader.read_i32().unwrap(),
            data_id: reader.read_i32().unwrap(),
            left: reader.read_i32().unwrap(),
            right: reader.read_i32().unwrap()
        });
    }

    for _i in 0..header.file_name_count{
        let buf = reader.read_bytes(0x80).unwrap();
        name_entries.push(NameEntry{
            extension: String::from_utf8(buf[..4].to_vec()).unwrap().replace("\0", "").replace(" ", ""),
            name: String::from_utf8(buf[4..].to_vec()).unwrap().replace("\0", "")
        });
    }

    for _i in 0..header.data_entry_count{
        data_entries.push(DataEntry{
            offset: reader.read_u64().unwrap(),
            size: reader.read_u64().unwrap(),
            comp_size: reader.read_u64().unwrap()
        });
    }

    let output_dir = Path::new(file_path).file_stem().unwrap();

    for i in 0..file_entries.len(){
        if file_entries[i].compare_bit == -1 && file_entries[i].data_id == -1{
            continue;
        }
        let data_entry = &data_entries[file_entries[i].data_id as usize];
        let name_entry = &name_entries[i];
        let file_path = [name_entry.name.clone(), name_entry.extension.clone()].join(".");
        let path = Path::new(output_dir).join(file_path);
        let file_dir = path.parent().unwrap();
        fs::create_dir_all(file_dir);

        println!("Extracting: {}", &path.display());

        reader.seek(SeekFrom::Start(data_entry.offset + header.data_start));
        let raw_data = reader.read_bytes(data_entry.comp_size as usize).unwrap();

        let r = File::create(&path)?;
        let mut writer = BinaryWriter::new(r, Options::default());
        if data_entry.size > data_entry.comp_size{
            let decompressed = decompress(&raw_data,  data_entry.size as usize).unwrap();
            writer.write_bytes(&decompressed);
        } else {
            writer.write_bytes(&raw_data);
        }
    }

    Ok(())
}