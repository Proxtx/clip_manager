use mp4::Result;
use openh264::{decoder, nal_units};
use std::fs::File;
use std::io::{BufRead, BufReader};

//mod mp4_bitstream_converter;

fn main() -> Result<()> {
    let f = File::open("example_data/video.mp4").unwrap();
    let size = f.metadata()?.len();
    let reader = BufReader::new(f);

    let mut mp4 = mp4::Mp4Reader::read_header(reader, size)?;

    println!(
        "brand: {}\ntimescale: {}\nsize: {}",
        mp4.ftyp.major_brand,
        mp4.moov.mvhd.timescale,
        mp4.size()
    );

    let mut compatible_brands = String::new();
    for brand in mp4.compatible_brands().iter() {
        compatible_brands.push_str(&brand.to_string());
        compatible_brands.push_str(",");
    }
    println!(
        "compatible brands: {}\nduration: {:?}",
        compatible_brands,
        mp4.duration()
    );

    for track in mp4.tracks().values() {
        println!(
            "track: #{}({}) {} : {}",
            track.track_id(),
            track.language(),
            track.track_type()?,
            track.media_type()?
        );
    }

    let track = mp4
        .tracks()
        .values()
        .find(|elem| elem.track_type().unwrap() == mp4::TrackType::Video)
        .unwrap();

    let track_id = track.track_id();

    let mut dec = decoder::Decoder::new().unwrap();

    let sample = mp4.read_sample(track_id, 1).unwrap().unwrap();

    let frm = dec.decode(&sample.bytes.split_at(30).0);
    println!("{} {:?}", sample.bytes.len(), frm);

    Ok(())
}
