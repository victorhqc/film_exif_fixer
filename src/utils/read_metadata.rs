use fraction::{error::ParseError, Fraction, ToPrimitive};
use regex::Regex;
use serde::Deserialize;
use snafu::prelude::*;
use std::path::Path;
use std::str::FromStr;

pub fn read_metadata(path: &Path) -> Result<Vec<ExposureInfo>, Error> {
    let mut rdr = csv::Reader::from_path(path).context(InvalidCSVSnafu)?;

    let mut res = Vec::new();
    for exp in rdr.deserialize() {
        let args: BuildExposureInfo = exp.context(FailedToParseSnafu)?;
        let exposure = ExposureInfo::build(args)?;

        res.push(exposure);
    }

    Ok(res)
}

#[derive(Debug)]
pub struct ExposureInfo {
    pub lens_name: String,
    pub focal_length: f32,
    pub date: String,
    pub iso: i32,
    pub aperture: f32,
    pub shutter_speed: String,
    pub exposure_compensation: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct BuildExposureInfo {
    lens_name: String,
    focal_length: f32,
    date: String,
    iso: i32,
    aperture: f32,
    shutter_speed: String,
    exposure_compensation: Option<String>,
}

impl ExposureInfo {
    pub fn build(args: BuildExposureInfo) -> Result<ExposureInfo, Error> {
        if !Self::is_shutter_speed_valid(&args.shutter_speed) {
            return Err(Error::InvalidShutterSpeed {
                text: args.shutter_speed.to_string(),
            });
        }

        let exp_comp = parse_exposure_compensation(&args.exposure_compensation)?;

        let result = ExposureInfo {
            lens_name: args.lens_name,
            date: args.date,
            iso: args.iso,
            focal_length: args.focal_length,
            aperture: args.aperture,
            shutter_speed: args.shutter_speed,
            exposure_compensation: exp_comp,
        };

        Ok(result)
    }

    fn is_shutter_speed_valid(txt: &str) -> bool {
        let expr = Regex::new(r#"1/\d+|\d+""#).unwrap();

        expr.is_match(txt)
    }
}

fn parse_exposure_compensation(
    exposure_compensation: &Option<String>,
) -> Result<Option<f32>, Error> {
    if let Some(exp_comp) = exposure_compensation {
        // This to allow the format of "1 1/3" or "2 2/3"
        let parts = exp_comp.split(" ").collect::<Vec<&str>>();
        if parts.len() > 2 {
            return Err(Error::InvalidExposureCompensation {
                value: exp_comp.clone(),
            });
        }

        let float: f32 = parts.iter().fold(0.0, |acc, part| {
            let float = Fraction::from_str(part)
                .context(ExposureCompensationSnafu {
                    value: exp_comp.clone(),
                })
                .unwrap();
            let float: f32 = float.to_f32().unwrap();

            if acc >= 0.0 {
                acc + float
            } else {
                acc - float
            }
        });

        Ok(Some(float))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("The Shutter speed is incorrect: {} does not follow the pattern", text))]
    InvalidShutterSpeed { text: String },

    #[snafu(display("Failed to read CSV: {:?}", source))]
    InvalidCSV { source: csv::Error },

    #[snafu(display("Failed to deserialize the row: {:?}", source))]
    FailedToParse { source: csv::Error },

    #[snafu(display("Wrong format for exposure compensation \"{}\": {:?}", value, source))]
    ExposureCompensation { source: ParseError, value: String },

    #[snafu(display("The format of the exposure compensation \"{}\" is wrong", value))]
    InvalidExposureCompensation { value: String },
}
