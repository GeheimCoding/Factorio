pub struct Animation {
    filename: FileName,
    filenames: Vec<FileName>,
    layers: Vec<Animation>,
    lines_per_file: u32,
    slice: u32,
    stripes: Vec<Stripe>,
}
