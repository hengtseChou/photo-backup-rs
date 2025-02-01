# photo-backup-rs

`photo-backup-rs` is a Rust implementation of [backup-iphone-photos.sh](https://github.com/hengtseChou/backup-iphone-photos.sh). This lightweight tool is designed for efficiently backing up iPhone photos, particularly those stored in directories like `iPhone/DCIM/101APPLE`. It can also work with other directory structures that include subfolders.

## Key Features

- **Organized backups:** Syncs photos from subfolders in the source directory to the destination, automatically sorting them into `YYYY-MM` subfolders.
- **Faster processing:** Skips already synced subfolders, assuming older ones remain unchanged.
- **No extra dependencies:** Includes `rsync` (version 3.4.1) within the project, eliminating the need for additional installations.

## Usage

This program syncs iPhone photos and organizes them into year-month folders.

```
Usage: photo-backup-rs [OPTIONS] --source <source> --dest <destination>

Options:
  -s, --source <source>     Path to the source directory
  -d, --dest <destination>  Path to the destination directory
  -l, --less                Reduce the amount of output
  -h, --help                Show help information
  -V, --version             Display version information
```

## License

This project is licensed under **GPL v3**.
