pub struct Post {
    // Check `request_review` to understand why it's Option
    // needed here
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
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_deref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // This if statement takes ownership of the state, which
        // means it's taking off the state and leaving a None value
        // instead. After that we can place a new state on the field.
        if let Some(s) = self.state.take() {
            // Update the state
            self.state = Some(s.request_review())
        }
    }
    
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    // "self: Box<Self>" means this method is only valid when
    // it's called from a Box<T>.
    //
    // This syntax takes ownership of Box<Self>, invalidating the
    // old state, which means the state value of Post can be transformed
    // into a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // Return empty by default
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    // Transfers the state from Draft to PendingReview
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // Returns draft because we cannot approve in this state
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // Nothing happens here, requesting a review will return
    // the same state
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Publised {})
    }
}

struct Publised {}

impl State for Publised {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
