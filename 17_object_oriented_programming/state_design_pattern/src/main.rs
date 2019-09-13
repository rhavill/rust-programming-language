mod oo_blog;
mod states_as_types_blog;

fn main() {
    oo_blog();
    states_as_types_blog();
}

fn oo_blog() {
    let mut post = oo_blog::Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    post.add_text("This text should be ignored");
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();

    post.add_text("This text should be ignored");
    assert_eq!("", post.content());

    post.approve();
    post.add_text("This text should be ignored");
    assert_eq!("I ate a salad for lunch today", post.content());
}

fn states_as_types_blog() {
    let mut post = states_as_types_blog::Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.reject();

    let post = post.request_review();

    let post = post.approve();

    let post = post.reject();

    let post = post.request_review();

    let post = post.approve();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}