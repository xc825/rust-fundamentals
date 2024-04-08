use clap::Parser;

#[allow(dead_code)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    TeraBytes(f64),
}

#[allow(dead_code)]
fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        1_000_000_000..= 999_999_999_999 => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
        _ => FileSize::TeraBytes(size as f64 / 1_000_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
        FileSize::TeraBytes(tb) => format!("{:.2} TB", tb),
    }
}

#[derive(Parser)]
struct Arguments {
    size: u64,
    unit: String
}

impl Arguments {
    fn format_size(&self) -> String {
        let size = match self.unit.as_str() {
            "B" => self.size as u64,
            "KB" => self.size as u64 * 1000,
            "MB" => self.size as u64 * 1_000_000,
            "GB" => self.size as u64 * 1_000_000_000,
            "TB" => self.size as u64 * 1_000_000_000_000,
            _ => self.size as u64,
        };
    
        let result = format!("Sizes {{ bytes: \"{} bytes\", kilobytes: \"{} kilobytes\", \
            megabytes: \"{} megabytes\", gigabytes: \"{} gigabytes\" }}",
                            size/1, size/1000, size/1_000_000, size/1_000_000_000);
        result
    }
}

//Sizes { bytes: "24000000 bytes", kilobytes: "24000 kilobytes", megabytes: "24 megabytes", gigabytes: "0 gigabytes" }
fn main() {
    let args = Arguments::parse();
    println!("size:{:?} unit:{:?}", args.size, args.unit);

    let result = args.format_size();
    println!("{}", result);

    //let args: Vec<String> = env::args().collect();
    let bytes :u64 = 6888837399453;
    let result = format_size(bytes);
    println!("{}", result);
}