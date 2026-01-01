use std::{fs};
use std::fs::File;
use std::io::Write;
use std::process::ExitCode;
use arboard::{Clipboard};
use clap::Parser;
use crossterm::style::Stylize;
use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
use photon_rs::native::{open_image, save_image};
use photon_rs::transform;
use photon_rs::transform::SamplingFilter;
use rayon::prelude::*;
use genicons::arguments::Arguments;
use genicons::manifest::{WebIcon, WebManifest};

const SIZE_NAME_PAIRS: [(&str, u32, u32); 5] = [
    ("android-chrome-192x192.png", 192, 192),
    ("android-chrome-512x512.png", 512, 512),
    ("apple-touch-icon.png", 180, 180),
    ("favicon-16.png", 16, 16),
    ("favicon-32.png", 32, 32)
];

fn main() -> ExitCode {
    let args = Arguments::parse();

    let master_image = match open_image(&args.master_image) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{}", format!("Failed to open image {:?}. {}", args.master_image, e).red());

            return ExitCode::FAILURE
        }
    };

    match fs::exists(&args.output_dir) {
        Ok(false) | Err(_) => {
            match fs::create_dir(&args.output_dir) {
                Err(e) => {
                    eprintln!("{}", format!("Failed to create output directory. {}", e).red());
                    return ExitCode::FAILURE;
                }
                _ => {}
            };
        }
        Ok(true) => {}
    }

    if !args.silent {
        println!("{}", "Generating icon files".green());
    }

    SIZE_NAME_PAIRS.into_par_iter().for_each(|(name, width, height)| {
        let resized = transform::resize(&master_image, width, height, SamplingFilter::CatmullRom);

        if let Err(e) = save_image(resized, &args.output_dir.join(name)) {
            eprintln!("Failed to save image {:?}. {}", name, e)
        }
    });

    if !args.silent {
        println!("{}", "Generating multi-size favicon.ico".green());
    }

    let mut out_ico = IconDir::new(ResourceType::Icon);

    for (width, height) in [(16, 16), (32, 32), (48, 48)] {
        let resized = transform::resize(&master_image, width, height, SamplingFilter::CatmullRom);

        let ico = IconImage::from_rgba_data(width, height, resized.get_raw_pixels());

        let entry = match IconDirEntry::encode(&ico) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("{}", format!("Failed to create ico entry for size {}x{}. {}", width, height, e));
                continue
            }
        };

        out_ico.add_entry(entry)
    }

    let out_file = match File::create(&args.output_dir.join("favicon.ico")) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", format!("Failed to create favicon.ico. {}", e));
            return ExitCode::FAILURE
        }
    };

    if let Err(e) = out_ico.write(out_file) {
        eprintln!("{}", format!("Failed to write favicon.ico. {}", e));
        return ExitCode::FAILURE
    }

    if !args.silent {
        println!("{}", "Generating manifest".green());
    }

    let icons = vec![
        WebIcon{
            src: "/android-chrome-192x192.png".to_string(),
            sizes: "192x192".to_string(),
            typ: "image/png".to_string(),
        },
        WebIcon{
            src: "/android-chrome-512x512.png".to_string(),
            sizes: "512x512".to_string(),
            typ: "image/png".to_string(),
        }
    ];
    let man = WebManifest{
        short_name: args.short_name.clone(),
        start_url: args.start_url.to_string(),
        display: "standalone".to_string(),
        theme_color: args.app_color.to_string(),
        name: args.app_name.to_string(),
        icons,
    };

    let js = match serde_json::to_string(&man) {
        Ok(js) => js,
        Err(e) => {
            eprintln!("{}", format!("Failed to create manifest.json. {}", e).red());
            return ExitCode::FAILURE
        }
    };

    let mut man_file = match File::create(&args.output_dir.join("manifest.json")) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", format!("Failed to create manifest.json. {}", e));
            return ExitCode::FAILURE
        }
    };

    if let Err(e) =man_file.write_all(js.as_bytes()) {
        eprintln!("{}", format!("Failed to write manifest.json. {}", e));
        return ExitCode::FAILURE
    }

    let html = r#"<link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png">
<link rel="icon" type="image/png" sizes="32x32" href="/favicon-32.png">
<link rel="icon" type="image/png" sizes="16x16" href="/favicon-16.png">
<link rel="manifest" href="/manifest.json">"#;

    if !loop {
        if args.no_html_copy {
            break false;
        }

        let mut clip = match Clipboard::new() {
            Ok(c) => c,
            Err(e) => {
                println!("{e}");
                eprintln!("{}", "Failed to setup clipboard".red());
                break false;
            }
        };

        if let Err(e) = clip.set_text(html) {
            eprintln!("{}", "Failed to write to clipboard".red());
            println!("{}", e);
            break false;
        }

        break true
    } {
        if !args.silent {
            println!("{}", "Clipboard disabled or failed. Copy this in the head of your html:".yellow());
            println!("{}", html)
        }
    } else {
        if !args.silent {
            println!("{}", "HTML Head copied.".green())
        }
    }

    if !args.silent {
        println!("{}", "Page meta generated!".green());
    }

    ExitCode::SUCCESS
}
