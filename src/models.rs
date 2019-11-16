use chrono::DateTime

pub struct Batch {
    id: u32,
    number: u32,
    name: String,
    created: DateTime,
}

pub struct Juice {
    id: u32,
    brand: String,
}

pub struct JuiceBatch {
    id: u32,
    juice: u32,
    purchased: DateTime,
}

pub enum YeastType {
    'solid',
    'liquid',
}

pub struct Yeast {
    id: u32,
    brand: String,
    name: String,
    type: YeastType,
}

pub struct YeastPacket {
    yeast: u32,
    expiration: DateTime,
    alreadyOpened: bool,
}

pub struct Bottling {
    id: u32,
    timestamp: DateTime,
    
}