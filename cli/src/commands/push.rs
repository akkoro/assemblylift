use std::fs;

use clap::ArgMatches;
use reqwest;
use reqwest::StatusCode;

use registry_common::models::{Coordinates, PutIomodAtRequest, RunConcatRequest, Version};

pub fn command(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for cast command"),
    };

    match matches.subcommand() {
        ("iomod", matches) => command_iomod(matches),
        _ => println!(
            "{}",
            "missing subcommand. try `asml pack help` for options."
        ),
    }
}

fn command_iomod(matches: Option<&ArgMatches>) {
    let matches = match matches {
        Some(matches) => matches,
        _ => panic!("could not get matches for cast command"),
    };

    let auth_header = matches.value_of("auth-header").unwrap(); // unwrap: this arg is required
    let package_path = matches.value_of("package").unwrap(); // unwrap: this arg is required
    let full_coords: Vec<&str> = matches.value_of("coords").unwrap().split('@').collect(); // unwrap: this arg is required
    let coords_string = String::from(full_coords[0]);
    let version_string = String::from(full_coords[1]);
    let coordinates: Vec<&str> = coords_string.split('.').collect();
    let version: Vec<&str> = version_string.split('.').collect();

    let package_bytes = fs::read(package_path).expect("unable to read package file");
    let part_size_bytes: usize = 2097152;
    let size_mb: f32 = package_bytes.len() as f32 / 1048576f32;
    println!("size_mb: {}", size_mb);

    if size_mb > (part_size_bytes as f32 / 1048576f32) {
        // TODO upload parts async
        let num_parts: u32 = (package_bytes.len() as f32 / part_size_bytes as f32).ceil() as u32;
        println!("num parts: {}", num_parts);
        for idx in 0usize..num_parts as usize {
            println!("uploading part #{}", idx);
            upload_part(
                coordinates.clone(),
                version.clone(),
                package_bytes[(idx * part_size_bytes)
                    ..std::cmp::min((idx + 1usize) * part_size_bytes, package_bytes.len())]
                    .to_vec(),
                auth_header.to_string(),
                Some(idx as u32),
            )
            .unwrap();
        }

        concat_parts(
            coordinates.clone(),
            version.clone(),
            auth_header.to_string(),
        )
        .unwrap();
    } else {
        upload_part(
            coordinates,
            version,
            package_bytes,
            auth_header.to_string(),
            None,
        )
        .unwrap();
    }
}

fn upload_part(
    coordinates: Vec<&str>,
    version: Vec<&str>,
    part_content: Vec<u8>,
    auth_header: String,
    part_id: Option<u32>,
) -> Result<(), ()> {
    let request = PutIomodAtRequest {
        coordinates: Coordinates {
            name: String::from(coordinates[2]),
            namespace: String::from(coordinates[1]),
            organization: String::from(coordinates[0]),
        },
        version: Version {
            major: version[0].parse::<u32>().unwrap(),
            minor: version[1].parse::<u32>().unwrap(),
            patch: version[2].parse::<u32>().unwrap(),
        },
        payload_part_id: part_id,
        payload_content_type: String::from("iomod/zip"), // TODO detect this
        payload_z85: z85::encode(part_content),
    };

    let client = reqwest::blocking::ClientBuilder::new()
        .build()
        .expect("could not build blocking HTTP client");
    let response = client
        .put("https://registry.assemblylift.akkoro.io/iomod")
        .header("Authorization", auth_header)
        .json(&request)
        .send()
        .expect("no response for: PUT registry.assemblylift.akkoro.io/iomod");

    match response.status() {
        StatusCode::OK => Ok(()),
        other => {
            println!("error: {:?}", other);
            println!("{:?}", response);
            Err(())
        }
    }
}

fn concat_parts(coordinates: Vec<&str>, version: Vec<&str>, auth_header: String) -> Result<(), ()> {
    let request = RunConcatRequest {
        coordinates: Coordinates {
            name: String::from(coordinates[2]),
            namespace: String::from(coordinates[1]),
            organization: String::from(coordinates[0]),
        },
        version: Version {
            major: version[0].parse::<u32>().unwrap(),
            minor: version[1].parse::<u32>().unwrap(),
            patch: version[2].parse::<u32>().unwrap(),
        },
    };

    let client = reqwest::blocking::ClientBuilder::new()
        .build()
        .expect("could not build blocking HTTP client");
    let response = client
        .post("https://registry.assemblylift.akkoro.io/iomod/concat")
        .header("Authorization", auth_header)
        .json(&request)
        .send()
        .expect("no response for: POST registry.assemblylift.akkoro.io/iomod/concat");

    match response.status() {
        StatusCode::OK => Ok(()),
        other => {
            println!("error: {:?}", other);
            Err(())
        }
    }
}
