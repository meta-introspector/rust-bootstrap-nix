pub trait IsUseStatement {
    fn get_usestatement_name(&self) -> &'static str;
}
