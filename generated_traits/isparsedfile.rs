pub trait IsParsedFile {
    fn get_parsedfile_name(&self) -> &'static str;
}
