use std::{process::Command, path::PathBuf, fs::read_dir, collections::HashMap};

use crate::device::{DeviceDescriptor, MountPoint};
use json::JsonValue;
use regex::Regex;

fn add_device_paths(drives:&mut Vec<DeviceDescriptor>)->anyhow::Result<()>
{
    let mut device_paths:HashMap<String,String>=HashMap::new();

    for res_dir in read_dir("/dev/disk/by-path/")?
    {
        let file=res_dir?;
        let path=file.path();

        if path.is_symlink() {
            let real_path=path.read_link()?;
            
            if real_path.is_absolute() {
                device_paths.insert(real_path.to_str().unwrap().to_string(), path.to_str().unwrap().to_string());
            }else{
                let filename=real_path.file_name().unwrap().to_str().unwrap();
                let mut paths=real_path.to_str().unwrap().split('/').collect::<Vec<&str>>();
                let mut p=PathBuf::from("/dev/disk/by-path");

                while let Some(val) = paths.pop() {
                    if val==".." {
                        p.pop();
                    }
                }

                p.push(filename);
                device_paths.insert(p.to_str().unwrap().to_string(), path.to_str().unwrap().to_string());
            }
        }
    }

    for drive in drives
    {
        if let Some(rel)=device_paths.get(&drive.device) {
            drive.devicePath=Some(rel.to_string());
        }
    }

    Ok(())
}

fn get_description(device:&JsonValue)->String
{
    let mut description=vec![device["label"].as_str().unwrap_or("").to_string(),device["vendor"].as_str().unwrap_or("").to_string(),device["model"].as_str().unwrap_or("").to_string()];
    let label=device["label"].as_str().unwrap_or("");

    let sub_labels=device["children"].members().filter(|c| if let Some(l)=c["label"].as_str(){
        l != label
    } else { false }).map(|c| c["label"].as_str().unwrap_or(c["mountpoint"].as_str().unwrap_or("")).to_string())
    .collect::<Vec<String>>();

    if sub_labels.len()>0 {
        description.push(sub_labels.join(", ").to_string());
    }

    Regex::new(r"\s+").unwrap().replace_all(description.join(" ").as_str(), " ").to_string()
}

fn get_mount_points(children:Vec<JsonValue>)->Vec<MountPoint>
{
    children.iter().filter(|c| c["mountpoint"].as_str().is_some()).map(|c| c.into()).collect::<Vec<MountPoint>>()
}

pub(crate)fn lsblk() -> anyhow::Result<Vec<DeviceDescriptor>>
{
    let output=Command::new("lsblk").args(["--bytes","-all","--json","--paths","--output-all"]).output()?;
    
    if let Some(code)=output.status.code() {
        if code != 0 {
            return Err(anyhow::Error::msg(format!("lsblk ExitCode: {}",code)));
        }
    }

    if output.stderr.len() > 0 {
        return Err(anyhow::Error::msg(format!("lsblk stderr: {}",std::str::from_utf8(&output.stderr).unwrap())));
    }

    let mut res=json::parse(std::str::from_utf8(&output.stdout).unwrap())?;

    for js_item in res["blockdevices"].members_mut()
    {
        js_item.insert("name", resolve_device_name(js_item["name"].as_str().unwrap_or("")))?;
        js_item.insert("kname", resolve_device_name(js_item["kname"].as_str().unwrap_or("")))?;
    }
    let re_block=Regex::new(r"^(block)$")?;
    let re_scsi=Regex::new(r"^(sata|scsi|ata|ide|pci)$")?;
    let re_usb=Regex::new(r"^(usb)$")?;

    let mut drives=res["blockdevices"].members().filter(|js|{
        let name=js["name"].as_str().unwrap_or("");

        !name.starts_with("/dev/loop") && !name.starts_with("/dev/sr") && !name.starts_with("/dev/ram")
    }).map(|js| {
        let mut device=DeviceDescriptor{
            enumerator:"lsblk:json".to_string(),
            busType:Some(js["tran"].as_str().unwrap_or("UNKNOWN").to_uppercase()),
            device:js["name"].as_str().unwrap_or("NO_NAME").to_string(),
            raw:js["kname"].as_str().unwrap_or(js["name"].as_str().unwrap_or("NO_NAME")).to_string(),
            isVirtual:re_block.is_match(js["subsystems"].as_str().unwrap_or("").to_lowercase().as_str()),
            isSCSI:re_scsi.is_match(js["tran"].as_str().unwrap_or("").to_lowercase().as_str()),
            isUSB:re_usb.is_match(js["tran"].as_str().unwrap_or("").to_lowercase().as_str()),
            isReadOnly:js["ro"].as_bool().unwrap_or(false),
            description:get_description(js),
            size:js["size"].as_i64().unwrap_or(0) as u64,
            blockSize:js["phy-sec"].as_i32().unwrap_or(512) as u32,
            logicalBlockSize:js["log-sec"].as_i32().unwrap_or(512) as u32,
            mountpoints:get_mount_points(if js.has_key("children"){
                js["children"].members().map(|c|c.clone()).collect::<Vec<JsonValue>>()
            }else { 
                vec![js.clone()]
            }),
            ..Default::default()
        };
        device.isRemovable=js["rm"].as_bool().unwrap_or(false)||js["hotplug"].as_bool().unwrap_or(false)||device.isVirtual;
        device.isSystem=!device.isRemovable && !device.isVirtual;

        if let Some(val) = js["pttype"].as_str() {
            device.partitionTableType=if val=="gpt"{
                Some("gpt".to_string())
            }else if val=="dos"{
                Some("mbr".to_string())
            }else{
                None
            };
        }
        device
    }).collect::<Vec<DeviceDescriptor>>();
    add_device_paths(&mut drives)?;
    Ok(drives)
}

fn resolve_device_name(name:&str)->String
{
    if name.is_empty() {
        return "".to_string();
    }

    let path=PathBuf::from(name);

    if !path.is_absolute() {
        format!("/dev{}",if name.starts_with("/") {
            &name[1..name.len()-1]
        }else{ name})
    } else {
        name.to_string()
    }
}