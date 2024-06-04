/// A structure for representing a post on a blogging site
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    /// Initializes the Post structure with an empty string and a Draft state
    /// State is an option so if a non existant post is referenced a Result is thrown
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    /// For adding text when the state is draft
    pub fn add_text(&mut self, text: &str) {
        // TODO: Make this only fire when state == Draft
        self.content.push_str(self.state.as_ref().unwrap().add_text(text));
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) =self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

trait State {
    // NOTE: default implementations violate object saftey
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn add_text<'a>(&self, text: &'a str) -> &'a str;
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn add_text<'a>(&self, text: &'a str) -> &'a str {
        text
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text<'a>(&self, text: &'a str) -> &'a str {
        ""
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PrePublication {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct PrePublication {}

impl State for PrePublication {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text<'a>(&self, text: &'a str) -> &'a str {
        ""
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft{})
    }
}

struct Published {}

impl State for Published {

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn add_text<'a>(&self, text: &'a str) -> &'a str {
        ""
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

