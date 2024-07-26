# ryzen-master-vbs-patch

New AMD Ryzen Master VBS patcher allows AMD Ryzen Master to run while Hyper-V Virtualization is enabled.

Thanks to [@TOM_RUS](https://twitter.com/TOM_RUS/status/1204867886197755904) for finding this.

This patch disables the VBS check on startup. It does NOT fix what might be the cause of this being disabled.

This MAY break functionality, and it MAY stop working on future versions.

This has been confirmed to work on the versions listed below so versions in between are likely fine as well.
The exact version does not have to match, but if AMD changes the code for detection it will likely break.

Confirmed versions:

* v2.1.0.1424 (2019)
* v2.1.1.1472 (2020)
* v2.2.0.1543 (2020)
* v2.3.0.1591 (2020)
* v2.6.0.1692 (2020)
* v2.6.2.1818 (2021)
* v2.8.0.1937 (2021)
* v2.13.0.2908 (2023)

2019-2021 patch made by [@klauspost](https://github.com/klauspost).

# Running

Various parts of this will require administrator access.

* Download and unzip binary from [releases](https://github.com/GuillaumeMCK/new-ryzen-master-vbs-patch/releases).
* Run the `ryzen-master-vbs-patch.exe` in administrator mode.
* If successful `Patched AMD Ryzen Master.exe` should now be created.
* Rename your existing `AMD Ryzen Master.exe` to `AMD Ryzen Master BACKUP.exe` or similar.
* Rename `Patched AMD Ryzen Master.exe` to `AMD Ryzen Master.exe`.

From the commandline, the syntax is:

```
Options:
-p, --path <PATH>      [default: "C:\Program Files\AMD\RyzenMaster\bin\AMD Ryzen Master.exe"]
-o, --output <OUTPUT>  [default: "C:\Program Files\AMD\RyzenMaster\bin\Patched AMD Ryzen Master.exe"]
-h, --help             Print help
-V, --version          Print version
```

# Building

Requires Cargo and Rust installed.

In the source directory run:

```bash
cargo build --release
```
