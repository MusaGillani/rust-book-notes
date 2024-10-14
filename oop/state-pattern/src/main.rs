use state_pattern::{typed_state_pattern::Post as TPost, Post};

fn main() {
    println!("State Objects approach");
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    println!("Typed State approach");
    let mut typed_post = TPost::new();

    typed_post.add_text("I ate a salad for lunch today");

    let typed_post = typed_post.request_review();

    let typed_post = typed_post.approve();

    assert_eq!("I ate a salad for lunch today", typed_post.content());
}
