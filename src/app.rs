use crate::image::gif::Gif;
use crate::image::Geometry;
use crate::record::Recorder;
use crate::util;
use chrono::Local;
use clap::ArgMatches;
use std::io::Error;

struct App {
    args: ArgMatches<'static>,
    recorder: Recorder,
    geometry: Geometry,
}

impl App {
    pub fn new(args: ArgMatches<'static>, geometry: Geometry) -> Result<Self, Error> {
        Ok(Self {
            args: args.clone(),
            recorder: Recorder::new(args
                .value_of("fps")
                .unwrap_or_default()
                .parse()
                .unwrap_or(10)),
            geometry
        })
    }
}