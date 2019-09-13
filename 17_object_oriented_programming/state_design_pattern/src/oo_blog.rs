pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(self.state.as_ref().unwrap().add_text(text));
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }   
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn add_text<'a>(&self, _text: &'a str) -> &'a str { 
        "" 
    }
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    /*
    Note that rather than having self, &self, or &mut self as the first 
    parameter of the method, we have self: Box<Self>. This syntax means the 
    method is only valid when called on a Box holding the type. This syntax 
    takes ownership of Box<Self>, invalidating the old state so the state value 
    of the Post can transform into a new state.
    */
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {has_approval: false})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn add_text<'a>(&self, text: &'a str) -> &'a str { 
        text 
    }
}

struct PendingReview {
    has_approval: bool,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.has_approval {
            Box::new(Published {})
        }
        else {
            Box::new(PendingReview{has_approval: true})
        }
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}