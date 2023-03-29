# Readme
This is basically a Rust implementation of [Balena's drivelist](https://github.com/balena-io-modules/drivelist).
At the moment, I don't own a Mac machine, so it only supports:

 - Windows
 - Linux

# Preparing

Issue command at your root project directory:

    cargo add "rs-drivelist"
  Or edit your `Cargo.toml` file, and add this line:

    rs-drivelist = "0.9.0"

## Usage

This library exports one function: rs_drivelist::drive_list() which returns a `Result` of `Vec<DeviceDescriptor>`

## Windows Output

    [{
           "enumerator": "SCSI",
           "busType": "NVME",
           "busVersion": "2.0",
           "device": "\\\\.\\PhysicalDrive0",
           "devicePath": null,
           "raw": "\\\\.\\PhysicalDrive0",
           "description": "SKHynix_HFM512GDHTNI-87A0B",
           "error": null,
           "partitionTableType": "gpt",
           "size": 512110190592,
           "blockSize": 4096,
           "logicalBlockSize": 512,
           "mountpoints": [
             {
                "path": "C:\\",
                "label": null,
                "totalBytes": 136773103616,
                "availableBytes": 24087683072
             },
             {
                 "path": "D:\\",
                 "label": null,
                 "totalBytes": 218398453760,
                 "availableBytes": 35988631552
             }
          ],
          "isReadOnly": false,
          "isSystem": true,
          "isCard": false,
          "isSCSI": false,
          "isUSB": false,
          "isVirtual": false,
          "isRemovable": false,
          "isUAS": false
    }]

