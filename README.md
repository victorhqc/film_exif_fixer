# Film Exif Fixer

## About

When scanning film negatives, the metadata of the exposures does not match.
This tool fixes that by replacing the EXIF metadata of the scans with its
correct values. It requires a CSV file with the information of each shot
to work.

## How I use it

Whenever I go out to shoot film, I take a picture with my phone of my camera
settings—that is, shutter speed and aperture. Then, at home, I write the
information in a spreadsheet. I note the date and time, ISO, shutter speed,
and aperture.

This information I use to correct the scans' EXIF metadata. When scanning, each
photo's EXIF metadata is from the digital camera, not the film. This script
replaces the key parts. The exposure information, focal length, and the camera
and lens names.

## How to use

_Note: This is a tool under development_

For now, this tool requires to be built locally. Check the development section
for more information on how to do it. Once it's built then continue on how to
use.

```shell

cargo run -- -s "<PATH_FOR_IMAGES>" -m "<PATH FOR CSV>.csv" -c <CAMERA_MODEL --maker <MAKER> -f <FILM_NAME>

# Example
cargo run -- -s "E:\Photos\Film Scans\2024\06-June" -m "E:\Photos\Film Scans\2024\06-June\metadata.csv" -c Hexar_RF --maker KONICA -f "Kodak Gold 200"
```

An important note: The number of rows of the CSV must match the number of photos
in the given file. Additionally, the images must be in a way that is sortable
by name, like: "1.jpg, 2.jpg ..." or "DSCF2470.RAF, DSCF2471.RAF ..." this way,
the CSV rows will match the images chronologically.

The CSV must be as follows

| no | lens_name              | focal_length | date                      | iso | aperture | shutter_speed |
|----|------------------------|--------------|---------------------------|-----|----------|---------------|
| 1  | 7Artisans 35mm f/2 MII | 35           | 2024:06:15 15:39:00+02:00 | 200 | 2.8      | 1/60          |

An example file can be found under the `fixtures/` path.

## Development

### Requirements

- Rust >= 1.79
- Perl >= 5
- Git >= 2.27

### For Windows

Make sure you have [chocolatey](https://chocolatey.org/) and install Perl

```bat
choco install strawberryperl
```

### How To Run

This depends on having [`exiftool`](https://exiftool.org/) installed. The following script will
download exiftool.

**For Unix Systems**

```bash
./scripts/unix/install.sh
```

**For Windows**

```bat
.\scripts\windows\install.bat
```