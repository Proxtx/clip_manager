use mp4::Result;
use openh264::decoder;
use std::fs::File;
use std::io::BufReader;

mod errors;
mod mp4_bitstream_converter;

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
        .find(|elem| elem.media_type().unwrap() == mp4::MediaType::H264)
        .unwrap();
    let track_id = track.track_id();

    let mut converter =
        mp4_bitstream_converter::Mp4BitstreamConverter::for_mp4_track(track).unwrap();
    let mut dec = decoder::Decoder::new().unwrap();

    let mut buffer = Vec::new();
    let sample = mp4.read_sample(track_id, 1).unwrap().unwrap();

    converter.convert_packet(&sample.bytes, &mut buffer);

    let dcy = dec.decode(&buffer).unwrap().unwrap();
    let dim_img = dcy.dimension_rgb();
    let mut buf_img = vec![0; dim_img.0 * dim_img.1 * 3];
    dcy.write_rgb8(&mut buf_img);

    println!("{:?}", buf_img);

    Ok(())
}
