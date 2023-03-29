use derivative::Derivative;
use json::{JsonValue,object};

#[derive(Debug,Default)]
pub struct MountPoint
{
    pub path:String,
    pub label:Option<String>,
    pub totalBytes:Option<u64>,
    pub availableBytes:Option<u64>
}

impl MountPoint
{
    pub fn new(path:impl ToString)->Self {
        Self { path: path.to_string(), label: None, totalBytes:None, availableBytes:None }
    }
}

impl Into<JsonValue> for &MountPoint
{
    fn into(self) -> JsonValue {
        object! {
            "path":self.path.clone(),
            "label":self.label.clone(),
            "totalBytes":self.totalBytes.clone(),
            "availableBytes":self.availableBytes.clone()
        }
    }
}

#[cfg(target_os="linux")]
impl From<&JsonValue> for MountPoint
{
    fn from(value: &JsonValue) -> Self {
        Self { 
            path: value["mountpoint"].as_str().unwrap_or("").to_string(), 
            label: if let Some(val) = value["label"].as_str() {
                val.to_string().into()
            } else {
                if let Some(val) = value["partlabel"].as_str() {
                    val.to_string().into()
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug,Derivative)]
#[derivative(Default)]
#[allow(non_snake_case)]
pub struct DeviceDescriptor {
    pub enumerator:String,
    pub busType:Option<String>,
    pub busVersion:Option<String>,
    pub device:String,
    pub devicePath:Option<String>,
    pub raw:String,
    pub description:String,
    pub error:Option<String>,
    pub partitionTableType:Option<String>,
    pub size:u64,
    #[derivative(Default(value="512"))]
    pub blockSize:u32,
    #[derivative(Default(value="512"))]
    pub logicalBlockSize:u32,
    pub mountpoints:Vec<MountPoint>,
    pub mountpointLabels:Vec<String>,
    /// Device is read-only
    pub isReadOnly:bool,
    /// Device is a system drive
    pub isSystem:bool,
    /// Device is an SD-card
    pub isCard:bool,
    /// Connected via the Small Computer System Interface (SCSI)
    pub isSCSI:bool,
    /// Connected via Universal Serial Bus (USB)
    pub isUSB:bool,
    /// Device is a virtual storage device
    pub isVirtual:bool,
    /// Device is removable from the running system
    pub isRemovable:bool,
    /// Connected via the USB Attached SCSI (UAS)
    pub isUAS:Option<bool>,
}

impl Into<JsonValue> for &DeviceDescriptor
{
    fn into(self) -> JsonValue {
        object! {
            "enumerator":self.enumerator.clone(),
            "busType":self.busType.clone(),
            "busVersion":self.busVersion.clone(),
            "device":self.device.clone(),
            "devicePath":self.devicePath.clone(),
            "raw":self.raw.clone(),
            "description":self.description.clone(),
            "error":self.error.clone(),
            "partitionTableType":self.partitionTableType.clone(),
            "size":self.size as i64,
            "blockSize":self.blockSize as i32,
            "logicalBlockSize":self.logicalBlockSize as i32,
            "mountpoints":self.mountpoints.iter().map(|c| {
                let val:JsonValue=c.into();
                val
            }).collect::<Vec<JsonValue>>(),
            "isReadOnly":self.isReadOnly.clone(),
            "isSystem":self.isSystem.clone(),
            "isCard":self.isCard.clone(),
            "isSCSI":self.isSCSI.clone(),
            "isUSB":self.isUSB.clone(),
            "isVirtual":self.isVirtual.clone(),
            "isRemovable":self.isRemovable.clone(),
            "isUAS":self.isUAS.clone()
        }
    }
}