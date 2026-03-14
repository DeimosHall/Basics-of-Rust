pub struct Post {
    content: String,
}

impl Post {
    // Create a draft post as initial "state"
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    // Only a final Post has this method available
    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String, // No method to get content here
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // We use "self" instead of "&self", by doing so we
    // consume the DraftPost and transform it into a
    // PendingReviewPost.
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            // Move the content
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
