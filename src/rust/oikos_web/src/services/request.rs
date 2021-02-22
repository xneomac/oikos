use crate::services::Error;
use yew::services::fetch::FetchTask;

#[derive(Default, Debug)]
pub struct Request {
    task: Option<FetchTask>,
    task_error: Option<Error>,
}

impl Request {
    pub fn new() -> Self {
        Self {
            task: None,
            task_error: None,
        }
    }

    #[allow(dead_code)]
    pub fn error(&self) -> Option<Error> {
        self.task_error.clone()
    }

    pub fn request(&mut self, task: FetchTask) {
        self.task = Some(task)
    }

    pub fn response<T>(&mut self, response: Result<T, Error>) -> Option<T> {
        match response {
            Ok(meal_plans) => {
                self.task_error = None;
                self.task = None;
                Some(meal_plans)
            }
            Err(err) => {
                self.task_error = Some(err);
                self.task = None;
                None
            }
        }
    }
}
