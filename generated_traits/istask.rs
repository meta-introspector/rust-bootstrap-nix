pub trait IsTask {
    fn get_task_name(&self) -> &'static str;
}
