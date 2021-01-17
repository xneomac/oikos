#![allow(clippy::ptr_arg)]


pub mod get_info {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use mockito::Matcher;
    use serde_json::json;
    use crate::models::*;
    use std::sync::Arc;
    pub struct MockBuilder200 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>,
        url: String,
    }

    impl MockBuilder200 {

        #[allow(clippy::new_without_default)]
        pub fn new(
        ) -> Self {
            let url =
                "/info".to_string();
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(),
                url,
            }
        }

        pub fn with_response(mut self, response_body: get_info::Response200,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: get_info::Response200, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();

            mockito::mock("GET", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                
                .with_status(200)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_200 (
        ) -> MockBuilder200 {
        MockBuilder200::new(
        )
    }
}

pub mod get_recipes {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use mockito::Matcher;
    use serde_json::json;
    use crate::models::*;
    use std::sync::Arc;
    pub struct MockBuilder200 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>,
        url: String,
    }

    impl MockBuilder200 {

        #[allow(clippy::new_without_default)]
        pub fn new(
        ) -> Self {
            let url =
                "/recipes".to_string();
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(),
                url,
            }
        }

        pub fn with_response(mut self, response_body: get_recipes::Response200,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: get_recipes::Response200, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();

            mockito::mock("GET", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                
                .with_status(200)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_200 (
        ) -> MockBuilder200 {
        MockBuilder200::new(
        )
    }
    pub struct MockBuilder401 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>,
        url: String,
    }

    impl MockBuilder401 {

        #[allow(clippy::new_without_default)]
        pub fn new(
        ) -> Self {
            let url =
                "/recipes".to_string();
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(),
                url,
            }
        }

        pub fn with_response(mut self, response_body: get_recipes::Response401,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: get_recipes::Response401, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();

            mockito::mock("GET", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                
                .with_status(401)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_401 (
        ) -> MockBuilder401 {
        MockBuilder401::new(
        )
    }
    pub struct MockBuilder403 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>,
        url: String,
    }

    impl MockBuilder403 {

        #[allow(clippy::new_without_default)]
        pub fn new(
        ) -> Self {
            let url =
                "/recipes".to_string();
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(),
                url,
            }
        }

        pub fn with_response(mut self, response_body: get_recipes::Response403,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: get_recipes::Response403, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();

            mockito::mock("GET", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                
                .with_status(403)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_403 (
        ) -> MockBuilder403 {
        MockBuilder403::new(
        )
    }
}
pub mod add_recipe {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use mockito::Matcher;
    use serde_json::json;
    use crate::models::*;
    use std::sync::Arc;
    pub struct MockBuilder200 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>, request_body: Option<add_recipe::Body>,
        url: String,
    }

    impl MockBuilder200 {

        #[allow(clippy::new_without_default)]
        pub fn new( body: Option<add_recipe::Body>,
        ) -> Self {
            let url =
                "/recipes".to_string();
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(), request_body: body,
                url,
            }
        }

        pub fn with_response(mut self, response_body: add_recipe::Response200,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: add_recipe::Response200, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();
            let request_body_matcher = match &self.request_body {
                Some(request_body) => Matcher::Json(json!(request_body)),
                None => Matcher::Any,
            };

            mockito::mock("POST", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                .match_body(request_body_matcher)
                .with_status(200)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_200 ( body: Option<add_recipe::Body>,
        ) -> MockBuilder200 {
        MockBuilder200::new(body,
        )
    }
}

pub mod get_recipe_by_id {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use mockito::Matcher;
    use serde_json::json;
    use crate::models::*;
    use std::sync::Arc;
    pub struct MockBuilder200 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>,
        url: String,
    }

    impl MockBuilder200 {

        #[allow(clippy::new_without_default)]
        pub fn new( parameters: &get_recipe_by_id::Parameters,
        ) -> Self {
            let url =
                format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id);
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(),
                url,
            }
        }

        pub fn with_response(mut self, response_body: get_recipe_by_id::Response200,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: get_recipe_by_id::Response200, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();

            mockito::mock("GET", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                
                .with_status(200)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_200 ( parameters: &get_recipe_by_id::Parameters,
        ) -> MockBuilder200 {
        MockBuilder200::new(parameters,
        )
    }
    pub struct MockBuilder401 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>,
        url: String,
    }

    impl MockBuilder401 {

        #[allow(clippy::new_without_default)]
        pub fn new( parameters: &get_recipe_by_id::Parameters,
        ) -> Self {
            let url =
                format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id);
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(),
                url,
            }
        }

        pub fn with_response(mut self, response_body: get_recipe_by_id::Response401,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: get_recipe_by_id::Response401, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();

            mockito::mock("GET", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                
                .with_status(401)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_401 ( parameters: &get_recipe_by_id::Parameters,
        ) -> MockBuilder401 {
        MockBuilder401::new(parameters,
        )
    }
    pub struct MockBuilder403 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>,
        url: String,
    }

    impl MockBuilder403 {

        #[allow(clippy::new_without_default)]
        pub fn new( parameters: &get_recipe_by_id::Parameters,
        ) -> Self {
            let url =
                format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id);
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(),
                url,
            }
        }

        pub fn with_response(mut self, response_body: get_recipe_by_id::Response403,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: get_recipe_by_id::Response403, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();

            mockito::mock("GET", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                
                .with_status(403)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_403 ( parameters: &get_recipe_by_id::Parameters,
        ) -> MockBuilder403 {
        MockBuilder403::new(parameters,
        )
    }
    pub struct MockBuilder404 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>,
        url: String,
    }

    impl MockBuilder404 {

        #[allow(clippy::new_without_default)]
        pub fn new( parameters: &get_recipe_by_id::Parameters,
        ) -> Self {
            let url =
                format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id);
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(),
                url,
            }
        }

        pub fn with_response(mut self, response_body: get_recipe_by_id::Response404,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: get_recipe_by_id::Response404, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();

            mockito::mock("GET", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                
                .with_status(404)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_404 ( parameters: &get_recipe_by_id::Parameters,
        ) -> MockBuilder404 {
        MockBuilder404::new(parameters,
        )
    }
}
pub mod update_recipe_by_id {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use mockito::Matcher;
    use serde_json::json;
    use crate::models::*;
    use std::sync::Arc;
    pub struct MockBuilder200 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>, request_body: Option<update_recipe_by_id::Body>,
        url: String,
    }

    impl MockBuilder200 {

        #[allow(clippy::new_without_default)]
        pub fn new( parameters: &update_recipe_by_id::Parameters, body: Option<update_recipe_by_id::Body>,
        ) -> Self {
            let url =
                format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id);
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(), request_body: body,
                url,
            }
        }

        pub fn with_response(mut self, response_body: update_recipe_by_id::Response200,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: update_recipe_by_id::Response200, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();
            let request_body_matcher = match &self.request_body {
                Some(request_body) => Matcher::Json(json!(request_body)),
                None => Matcher::Any,
            };

            mockito::mock("PUT", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                .match_body(request_body_matcher)
                .with_status(200)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_200 ( parameters: &update_recipe_by_id::Parameters, body: Option<update_recipe_by_id::Body>,
        ) -> MockBuilder200 {
        MockBuilder200::new(parameters,body,
        )
    }
    pub struct MockBuilder401 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>, request_body: Option<update_recipe_by_id::Body>,
        url: String,
    }

    impl MockBuilder401 {

        #[allow(clippy::new_without_default)]
        pub fn new( parameters: &update_recipe_by_id::Parameters, body: Option<update_recipe_by_id::Body>,
        ) -> Self {
            let url =
                format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id);
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(), request_body: body,
                url,
            }
        }

        pub fn with_response(mut self, response_body: update_recipe_by_id::Response401,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: update_recipe_by_id::Response401, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();
            let request_body_matcher = match &self.request_body {
                Some(request_body) => Matcher::Json(json!(request_body)),
                None => Matcher::Any,
            };

            mockito::mock("PUT", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                .match_body(request_body_matcher)
                .with_status(401)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_401 ( parameters: &update_recipe_by_id::Parameters, body: Option<update_recipe_by_id::Body>,
        ) -> MockBuilder401 {
        MockBuilder401::new(parameters,body,
        )
    }
    pub struct MockBuilder403 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>, request_body: Option<update_recipe_by_id::Body>,
        url: String,
    }

    impl MockBuilder403 {

        #[allow(clippy::new_without_default)]
        pub fn new( parameters: &update_recipe_by_id::Parameters, body: Option<update_recipe_by_id::Body>,
        ) -> Self {
            let url =
                format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id);
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(), request_body: body,
                url,
            }
        }

        pub fn with_response(mut self, response_body: update_recipe_by_id::Response403,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: update_recipe_by_id::Response403, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();
            let request_body_matcher = match &self.request_body {
                Some(request_body) => Matcher::Json(json!(request_body)),
                None => Matcher::Any,
            };

            mockito::mock("PUT", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                .match_body(request_body_matcher)
                .with_status(403)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_403 ( parameters: &update_recipe_by_id::Parameters, body: Option<update_recipe_by_id::Body>,
        ) -> MockBuilder403 {
        MockBuilder403::new(parameters,body,
        )
    }
    pub struct MockBuilder404 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>, request_body: Option<update_recipe_by_id::Body>,
        url: String,
    }

    impl MockBuilder404 {

        #[allow(clippy::new_without_default)]
        pub fn new( parameters: &update_recipe_by_id::Parameters, body: Option<update_recipe_by_id::Body>,
        ) -> Self {
            let url =
                format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id);
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(), request_body: body,
                url,
            }
        }

        pub fn with_response(mut self, response_body: update_recipe_by_id::Response404,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: update_recipe_by_id::Response404, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();
            let request_body_matcher = match &self.request_body {
                Some(request_body) => Matcher::Json(json!(request_body)),
                None => Matcher::Any,
            };

            mockito::mock("PUT", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                .match_body(request_body_matcher)
                .with_status(404)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_404 ( parameters: &update_recipe_by_id::Parameters, body: Option<update_recipe_by_id::Body>,
        ) -> MockBuilder404 {
        MockBuilder404::new(parameters,body,
        )
    }
}
pub mod delete_recipe_by_id {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use mockito::Matcher;
    use serde_json::json;
    use crate::models::*;
    use std::sync::Arc;
    pub struct MockBuilder200 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>,
        url: String,
    }

    impl MockBuilder200 {

        #[allow(clippy::new_without_default)]
        pub fn new( parameters: &delete_recipe_by_id::Parameters,
        ) -> Self {
            let url =
                format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id);
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(),
                url,
            }
        }

        pub fn with_response(mut self, response_body: delete_recipe_by_id::Response200,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: delete_recipe_by_id::Response200, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();

            mockito::mock("DELETE", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                
                .with_status(200)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_200 ( parameters: &delete_recipe_by_id::Parameters,
        ) -> MockBuilder200 {
        MockBuilder200::new(parameters,
        )
    }
    pub struct MockBuilder401 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>,
        url: String,
    }

    impl MockBuilder401 {

        #[allow(clippy::new_without_default)]
        pub fn new( parameters: &delete_recipe_by_id::Parameters,
        ) -> Self {
            let url =
                format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id);
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(),
                url,
            }
        }

        pub fn with_response(mut self, response_body: delete_recipe_by_id::Response401,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: delete_recipe_by_id::Response401, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();

            mockito::mock("DELETE", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                
                .with_status(401)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_401 ( parameters: &delete_recipe_by_id::Parameters,
        ) -> MockBuilder401 {
        MockBuilder401::new(parameters,
        )
    }
    pub struct MockBuilder403 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>,
        url: String,
    }

    impl MockBuilder403 {

        #[allow(clippy::new_without_default)]
        pub fn new( parameters: &delete_recipe_by_id::Parameters,
        ) -> Self {
            let url =
                format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id);
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(),
                url,
            }
        }

        pub fn with_response(mut self, response_body: delete_recipe_by_id::Response403,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: delete_recipe_by_id::Response403, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();

            mockito::mock("DELETE", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                
                .with_status(403)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_403 ( parameters: &delete_recipe_by_id::Parameters,
        ) -> MockBuilder403 {
        MockBuilder403::new(parameters,
        )
    }
    pub struct MockBuilder404 {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>,
        url: String,
    }

    impl MockBuilder404 {

        #[allow(clippy::new_without_default)]
        pub fn new( parameters: &delete_recipe_by_id::Parameters,
        ) -> Self {
            let url =
                format!("/recipes/{recipe_id}", recipe_id = parameters.recipe_id);
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(),
                url,
            }
        }

        pub fn with_response(mut self, response_body: delete_recipe_by_id::Response404,) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: delete_recipe_by_id::Response404, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();

            mockito::mock("DELETE", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                
                .with_status(404)
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock_404 ( parameters: &delete_recipe_by_id::Parameters,
        ) -> MockBuilder404 {
        MockBuilder404::new(parameters,
        )
    }
}
