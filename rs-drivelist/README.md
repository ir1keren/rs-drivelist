# rs-drivelist
A high-performance, cross-platform Rust library to list all connected storage devices.
Inspired by [Balena's drivelist](https://github.com/balena-io-modules/drivelist), this crate provides a native Rust implementation to identify physical drives, their mount points, and metadata without the overhead of a heavy runtime.


[![crates.io](https://img.shields.io/crates/v/rs-drivelist?label=latest)](https://crates.io/crates/rs-drivelist) ![MSRV](https://img.shields.io/badge/rustc-1.59+-ab6000.svg) ![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/actix-web.svg)

## 🚀 Features
**Cross-Platform**: Robust support for Linux and Windows.
**Detailed Metadata**: Retrieve drive names, sizes, mount points, and whether a drive is removable or system-owned.
**Native Performance**: Written in pure Rust for safety and speed.
**No External Dependencies**: Minimalist approach to keep your binary small.

## 📦 Installation
Add this to your ``Cargo.toml``:
```toml
[dependencies]
rs-drivelist = "0.1.0" # Replace with latest version
```

## 💻 Usage
```rust
use rs_drivelist::list;

fn main() {
    match list() {
        Ok(drives) => {
            for drive in drives {
                println!("Device: {}", drive.device);
                println!("Description: {}", drive.description);
                println!("Size: {} bytes", drive.size);
                println!("Mountpoints: {:?}", drive.mountpoints);
                println!("-------------------");
            }
        }
        Err(e) => eprintln!("Error retrieving drives: {}", e),
    }
}
```

## 🛠 Platform Support OS
| **OS**    | **Method**                        | **Status**    |
|-----------|-----------------------------------|---------------|
| *Linux*   | ``util-linux``  /  ``/sys/block`` | ✅ Supported   |
| *Windows* | Logical & Physical Drive APIs     | ✅ Supported   |
| *macOS*   | Objective C call                  | ☢️ In Progress |

## 🤝 Contributing
Contributions are welcome! If you find a bug or want to add support for a new platform (like macOS), please feel free to open an issue or submit a pull request.

## 📄 License
This project is licensed under the MIT License.

## Donation
My main laptop I used to start this project is broken, and I'm using a Celeron N2840 with unupgradable soldered 2GB RAM.
Your donation will be very much appreciated, as my bank account couldn't bought me a proper machine.
Visit me on my [Ko-fi account](https://ko-fi.com/ir1keren)