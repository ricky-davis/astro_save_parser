
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use flate2::read::ZlibDecoder;

use byteorder::{ByteOrder, LittleEndian};


static DECOMPRESSED_DIRECTORY: &str = "saves/decompressed";

#[derive(Debug)]
pub struct TempSaveData {
    filename: String,
    file: Option<File>,
    filesize: u64,
    header: Vec<u8>,
    compressed_data: Vec<u8>,
    decompressed_data: Vec<u8>,
    cursor: u64
}
impl TempSaveData {
    pub fn new_bytes(fileName: String, fileSize: u64, fileData: &[u8]) -> TempSaveData {
        let filename = fileName;
        let filesize = fileSize;
        let file = None;
        
        TempSaveData{
            filename: filename,
            file: file,
            filesize: filesize,
            header: Vec::new(),
            compressed_data: fileData.to_vec(),
            decompressed_data: Vec::new(),
            cursor: 0,
        }
    }

    pub fn get_next_x(&mut self, x: u64) -> Vec<u8>{
        let size = x;
        let d: Vec<_> = self.decompressed_data[(self.cursor as usize)..((self.cursor+size) as usize)].to_vec();
        self.cursor+=size;
        return d;
    }

    pub fn get_next_u8(&mut self) -> u8{
        let size = 1;
        let d: Vec<_> = self.decompressed_data[(self.cursor as usize)..((self.cursor+size) as usize)].to_vec();
        self.cursor+=size;
        let rtn = *d.get(0).unwrap();
        // println!("{:?}",rtn);
        return rtn;
    }
    pub fn get_next_u16(&mut self) -> u16{
        let size = 2;
        let d: Vec<_> = self.decompressed_data[(self.cursor as usize)..((self.cursor+size) as usize)].to_vec();
        self.cursor+=size;
        let rtn = LittleEndian::read_u16(&d);
        // println!("{:?}",rtn);
        return rtn;
    }
    pub fn get_next_i16(&mut self) -> i16{
        let size = 2;
        let d: Vec<_> = self.decompressed_data[(self.cursor as usize)..((self.cursor+size) as usize)].to_vec();
        self.cursor+=size;
        let rtn =  LittleEndian::read_i16(&d);
        // println!("{:?}",rtn);
        return rtn;
    }
    pub fn get_next_u32(&mut self) -> u32{
        let size = 4;
        let d: Vec<_> = self.decompressed_data[(self.cursor as usize)..((self.cursor+size) as usize)].to_vec();
        self.cursor+=size;
        let rtn =  LittleEndian::read_u32(&d);
        // println!("{:?}",rtn);
        return rtn;
    }
    pub fn get_next_i32(&mut self) -> i32{
        let size = 4;
        let d: Vec<_> = self.decompressed_data[(self.cursor as usize)..((self.cursor+size) as usize)].to_vec();
        self.cursor+=size;
        let rtn =  LittleEndian::read_i32(&d);
        // println!("{:?}",rtn);
        return rtn;
    }
    pub fn get_next_u64(&mut self) -> u64{
        let size = 8;
        let d: Vec<_> = self.decompressed_data[(self.cursor as usize)..((self.cursor+size) as usize)].to_vec();
        self.cursor+=size;
        let rtn =  LittleEndian::read_u64(&d);
        // println!("{:?}",rtn);
        return rtn;
    }
    pub fn get_next_i64(&mut self) -> i64{
        let size = 8;
        let d: Vec<_> = self.decompressed_data[(self.cursor as usize)..((self.cursor+size) as usize)].to_vec();
        self.cursor+=size;
        let rtn =  LittleEndian::read_i64(&d);
        // println!("{:?}",rtn);
        return rtn;
    }
    pub fn get_next_u128(&mut self) -> u128{
        let size = 16;
        let d: Vec<_> = self.decompressed_data[(self.cursor as usize)..((self.cursor+size) as usize)].to_vec();
        self.cursor+=size;
        let rtn =  LittleEndian::read_u128(&d);
        // println!("{:?}",rtn);
        return rtn;
    }
    pub fn get_next_f32(&mut self) -> f32{
        let size = 4;
        let d: Vec<_> = self.decompressed_data[(self.cursor as usize)..((self.cursor+size) as usize)].to_vec();
        self.cursor+=size;
        let rtn =  LittleEndian::read_f32(&d);
        // println!("{:?}",rtn);
        return rtn;
    }
    pub fn get_next_string(&mut self) -> String{
        let s: Vec<_> = self.decompressed_data[(self.cursor as usize)..((self.cursor+4) as usize)].to_vec();
        let str_size = LittleEndian::read_i32(&s) as u64;
        // println!("{:?}",&str_size);
        self.cursor+=4;

        let d: Vec<_> = self.decompressed_data[(self.cursor as usize)..((self.cursor+str_size) as usize)].to_vec();
        // println!("{:?}",&d);
        self.cursor+=str_size;
        
        let rtn =  String::from_utf8(d).expect("Found invalid UTF-8");
        // println!("{:?}",rtn);
        return rtn;
    }
}
#[derive(Debug)]
pub struct Save {
    name: String,
    temp_save_data: Option<TempSaveData>,
    structured_data: Option<DecompressedData>
}

impl Save {
    pub fn new_bytes(fileName: String, fileSize: u64, fileData: &[u8]) -> Save {
        let mut tsd_o = Some(TempSaveData::new_bytes(fileName.clone(),fileSize,&fileData.to_vec()));
        Save {
            name: fileName,
            temp_save_data: tsd_o,
            structured_data: None
        }
    }
    pub fn load(&mut self){
        // read first 16 bytes as header
        
        let mut tsd: &mut TempSaveData = self.temp_save_data.as_mut().unwrap();
        let mut compressed = Vec::new();

        let header: Vec<_> = tsd.compressed_data.drain(0..16).collect();
        println!("Header: {:?}",&header);

        tsd.header = header.to_vec();
        
        // read everything else as zlib compressed data
        compressed = (*tsd.compressed_data).to_vec();
        println!("Compr: {:?}",&compressed[0..16]);
        let original_size = &compressed.len();
        tsd.compressed_data = compressed.clone();
    
        let mut decompressed = Vec::new();
        ZlibDecoder::new(&compressed[..]).read_to_end(&mut decompressed).expect("decompressing file");
        tsd.compressed_data = Vec::new();
        tsd.decompressed_data = decompressed.clone();

        println!("Decompressed from {0} to {1} bytes", original_size, decompressed.len());

        println!("Deserializing data...");

        self.structured_data = Some(DecompressedData::deserialize(&mut tsd));
        self.temp_save_data = None;
        // println!("{:?}",self.structured_data);
    }
}

#[derive(Debug)]
struct DecompressedData {
    header: Header,
    astro_save: AstroSave
}

impl DecompressedData {
    pub fn deserialize(save: &mut TempSaveData) -> DecompressedData{
        let h = Header::deserialize(save);
        println!("here1");
        let s = AstroSave::deserialize(save);
        DecompressedData{
            header: h,
            astro_save: s
        }
    }
}


#[derive(Debug)]
struct Header {
    format_tag: u32,
    save_game_version: i32,
    package_version: i32,
    engine_version: EngineVersion,
    custom_format_data: CustomFormatData,
    save_class: String,
    end_of_header1: String,
    end_of_header2: i32
}

impl Header {
    pub fn deserialize(save: &mut TempSaveData) -> Header{
        Header{
            format_tag: save.get_next_u32(),
            save_game_version: save.get_next_i32(),
            package_version: save.get_next_i32(),
            engine_version: EngineVersion::deserialize(save),
            custom_format_data: CustomFormatData::deserialize(save),
            save_class: save.get_next_string(),
            end_of_header1: save.get_next_string(),
            end_of_header2: save.get_next_i32()
        }
    }
}

#[derive(Debug)]
struct EngineVersion {
    major: u16,
    minor: u16,
    patch: u16,
    build: u32,
    build_id: String
}

impl EngineVersion {
    pub fn deserialize(save: &mut TempSaveData) -> EngineVersion{
        EngineVersion{
            major: save.get_next_u16(),
            minor: save.get_next_u16(),
            patch: save.get_next_u16(),
            build: save.get_next_u32(),
            build_id: save.get_next_string()
        }
    }
}

#[derive(Debug)]
struct CustomFormatData{
    version: i32,
    custom_format_count: u32,
    custom_format_datum: Vec<CustomFormatDatum>
}

impl CustomFormatData {
    pub fn deserialize(save: &mut TempSaveData) -> CustomFormatData{
        let vers = save.get_next_i32();
        let count = save.get_next_u32();
        let mut cfd = Vec::new();
        for x in 0..count{
            cfd.push(CustomFormatDatum::deserialize(save));
        }
        CustomFormatData{
            version: vers,
            custom_format_count: count,
            custom_format_datum: cfd
        }
    }
}

#[derive(Debug)]
struct CustomFormatDatum{
    id: u128, // Guid
    value: i32
}
impl CustomFormatDatum {
    pub fn deserialize(save: &mut TempSaveData) -> CustomFormatDatum{
        CustomFormatDatum{
            id: save.get_next_u128(),
            value: save.get_next_i32()
        }
    }
}


///////////////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug)]
struct AstroSave {
    level_chunk: AstroLevelSaveChunk,
    remote_player_chunks_count: i32,
    remote_player_chunks: Vec<AstroRemotePlayerChunk> 
}

impl AstroSave {
    pub fn deserialize(save: &mut TempSaveData) -> AstroSave{
        let alsc = AstroLevelSaveChunk::deserialize(save);
        let rpc_count = save.get_next_i32();
        let mut rpc = Vec::new();
        for x in 0..rpc_count{
            rpc.push(AstroRemotePlayerChunk::deserialize(save));
        }
        AstroSave{
            level_chunk: alsc,
            remote_player_chunks_count: rpc_count,
            remote_player_chunks: rpc
        }
    }
}



#[derive(Debug)]
struct AstroLevelSaveChunk {
    astro_save_version: u32,
    level_name: String,
    data: AstroSaveChunk, 
    player_controller_records_count: i32,
    player_controller_records: Vec<PlayerControllerRecord>
}

impl AstroLevelSaveChunk {
    pub fn deserialize(save: &mut TempSaveData) -> AstroLevelSaveChunk{
        let asv = save.get_next_u32();
        let ln = save.get_next_string();
        let data = AstroSaveChunk::deserialize(save);
        let pcr_count = save.get_next_i32();
        let mut pcr = Vec::new();
        for x in 0..pcr_count{
            pcr.push(PlayerControllerRecord::deserialize(save));
        }
        AstroLevelSaveChunk{
            astro_save_version: asv,
            level_name: ln,
            data: data, 
            player_controller_records_count: pcr_count,
            player_controller_records: pcr
        }
    }
}



#[derive(Debug)]
struct PlayerControllerRecord {
    actor_index: u32,
    last_controller_pawn: u32,
    network_uuid: u64
}

impl PlayerControllerRecord {
    pub fn deserialize(save: &mut TempSaveData) -> PlayerControllerRecord{
        PlayerControllerRecord{
            actor_index: save.get_next_u32(),
            last_controller_pawn: save.get_next_u32(),
            network_uuid: save.get_next_u64()
        }
    }
}



#[derive(Debug)]
struct AstroRemotePlayerChunk {
    data : AstroSaveChunk,
    network_uuid: u64
}

impl AstroRemotePlayerChunk {
    pub fn deserialize(save: &mut TempSaveData) -> AstroRemotePlayerChunk{
        AstroRemotePlayerChunk{
            data : AstroSaveChunk::deserialize(save),
            network_uuid: save.get_next_u64()
        }
    }
}



#[derive(Debug)]
struct AstroSaveChunk {
    astro_save_version: u32,
    names: StringTable,
    object_records_count: u32,
    object_records: Vec<ObjectSaveRecord>, 
    actor_records_count: u32,
    actor_records: Vec<ActorRecord>,
    root_level_actor_indices_count: u32,
    root_level_actor_indices: Vec<i32>,
    first_import_index: u32
}

impl AstroSaveChunk {
    pub fn deserialize(save: &mut TempSaveData) -> AstroSaveChunk{
        let asv = save.get_next_u32();
        let names = StringTable::deserialize(save);
        let or_count = save.get_next_u32();
        let mut or = Vec::new();
        for x in 0..or_count{
            or.push(ObjectSaveRecord::deserialize(save));
        }
        let ar_count = save.get_next_u32();
        let mut ar = Vec::new();
        for x in 0..ar_count{
            ar.push(ActorRecord::deserialize(save));
        }
        let rlai_count = save.get_next_u32();
        let mut rlai = Vec::new();
        for x in 0..rlai_count{
            rlai.push(save.get_next_i32());
        }
        let fii = save.get_next_u32();

        AstroSaveChunk{
            astro_save_version: asv,
            names: names,
            object_records_count: or_count,
            object_records: or, 
            actor_records_count: ar_count,
            actor_records: ar,
            root_level_actor_indices_count: rlai_count,
            root_level_actor_indices: rlai,
            first_import_index: fii
        }
    }
}



#[derive(Debug)]
struct StringTable {
    count: i64,
    strings: Vec<String>
}
impl StringTable {
    pub fn deserialize(save: &mut TempSaveData) -> StringTable{
        let count = save.get_next_i64();
        let mut strings = Vec::new();
        for x in 0..(count-1){
            strings.push(save.get_next_string());
        }
        StringTable{
            count: count,
            strings: strings
        }
    }
}


#[derive(Debug)]
struct ObjectSaveRecord {
    object_type: String,
    name_index: i32,
    flags: u32,
    save_flags: u8,
    outer_object_index: i32, // parent
    custom_data_offset:u32,
    size:u32,
    data: Vec<u8>,  // length is size
    custom_data: Vec<u8> // length is size - custom_data_offset
}
impl ObjectSaveRecord {
    pub fn deserialize(save: &mut TempSaveData) -> ObjectSaveRecord{
        let ot = save.get_next_string();
        let ni = save.get_next_i32();
        let flags = save.get_next_u32();
        let save_flags = save.get_next_u8();
        let ooi = save.get_next_i32();
        let cdo = save.get_next_u32();
        let mut size: u32 = 0;
        if ((save_flags & 4) != 0){
            size = save.get_next_u32();
        }
        
        // println!("save_flags: {:?}",&save_flags);
        // println!("save_flags&4: {:?}",&save_flags&4);
        // println!("CDO: {:?}",&cdo);
        // println!("SIZE: {:?}",&size);

        let data = save.get_next_x(cdo as u64);
        // println!("datalen: {:?}",&data.len());
        let mut custom_data = Vec::new();
        if (size != 0){
            let tSize = (size-cdo) as u64;
            custom_data = save.get_next_x(tSize);
        }
        // println!("cdatalen: {:?}",&custom_data.len());

        ObjectSaveRecord{
            object_type: ot,
            name_index: ni,
            flags: flags,
            save_flags: save_flags,
            outer_object_index: ooi,
            custom_data_offset:cdo,
            size:size,
            data: data,
            custom_data: custom_data
        }
    }
}

#[derive(Debug)]
struct ActorRecord {
    object_index: i32,
    child_actor_count: i32,
    child_actor_records: Vec<ChildActorRecord>,
    owned_component_count: i32,
    owned_components: Vec<ComponentRecord>,
    root_transform: Transform
}
impl ActorRecord {
    pub fn deserialize(save: &mut TempSaveData) -> ActorRecord{
        let oi = save.get_next_i32();
        let ca_count = save.get_next_i32();
        let mut car = Vec::new();
        for x in 0..ca_count{
            car.push(ChildActorRecord::deserialize(save));
        }
        let oc_count = save.get_next_i32();
        let mut oc = Vec::new();
        for x in 0..oc_count{
            oc.push(ComponentRecord::deserialize(save));
        }
        let tnsfm =  Transform::deserialize(save);

        ActorRecord{
            object_index: oi,
            child_actor_count: ca_count,
            child_actor_records: car,
            owned_component_count: oc_count,
            owned_components: oc,
            root_transform: tnsfm
        }
    }
}

#[derive(Debug)]
struct ChildActorRecord {
    name_index: i32,
    actor_index: i32
}
impl ChildActorRecord {
    pub fn deserialize(save: &mut TempSaveData) -> ChildActorRecord{
        ChildActorRecord{
            name_index: save.get_next_i32(),
            actor_index: save.get_next_i32()
        }
    }
}


#[derive(Debug)]
struct ComponentRecord {
    name_index: i32,
    object_index: i32
}
impl ComponentRecord {
    pub fn deserialize(save: &mut TempSaveData) -> ComponentRecord{
        ComponentRecord{
            name_index: save.get_next_i32(),
            object_index: save.get_next_i32()
        }
    }
}

#[derive(Debug)]
struct Transform {
    rotation: Quaternion,
    translation: Vec<f32>,
    scale: Vec<f32>
}
impl Transform {
    pub fn deserialize(save: &mut TempSaveData) -> Transform{
        let quat = Quaternion::deserialize(save);
        let mut trns = Vec::new();
        for x in 0..3{
            trns.push(save.get_next_f32());
        }
        let mut scale = Vec::new();
        for x in 0..3{
            scale.push(save.get_next_f32());
        }
        Transform{
            rotation: quat,
            translation: trns,
            scale: scale
        }
    }
}

#[derive(Debug)]
struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}
impl Quaternion {
    pub fn deserialize(save: &mut TempSaveData) -> Quaternion{
        Quaternion{
            x: save.get_next_f32(),
            y: save.get_next_f32(),
            z: save.get_next_f32(),
            w: save.get_next_f32()
        }
    }
}