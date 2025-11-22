use oop_blog_post::Post;

fn main() {
    // one way
    // let mut post = Post::new();
    // post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());
    // post.add_text("hello");
    //
    // post.request_review();
    // assert_eq!("", post.content());
    // post.add_text("none");
    // post.approve();
    // post.add_text("after approve");
    // assert_eq!("I ate a salad for lunch today", post.content());

    // another way
    let mut post = Post::new("");
    post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());
    let req_post = post.request_review();
    // assert_eq!("", post.content());
    let approve_post = req_post.approve();

    assert_eq!("I ate a salad for lunch today", approve_post.content());
}
