// use crate::traditional_blog::Post;

use crate::rust_style_blog::Post;

mod traditional_blog;
mod rust_style_blog;

fn main() {
    // let mut post = Post::new();
    
    // post.add_text("Hello, this is a post!");
    // assert_eq!("", post.content());
    
    // post.request_review();   
    // assert_eq!("", post.content());
    
    // post.approve();
    // assert_eq!("Hello, this is a post!", post.content());
    
    let mut post = Post::new();
    post.add_text("This is the rust way!");
    // By using the rust way, we don't even have access
    // to content() until the post is approved, check the source
    // code.
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("This is the rust way!", post.content());
}
