pub trait IsRawFile {
    fn get_rawfile_name(&self) -> &'static str;
}
