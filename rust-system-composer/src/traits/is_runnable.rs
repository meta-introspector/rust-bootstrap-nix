pub trait IsRunnable {
    fn is_dry_run(&self) -> bool;
    fn set_dry_run(&mut self, dry_run: bool);
}