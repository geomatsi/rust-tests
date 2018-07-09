extern crate blog;

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Hello World");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Hello World", post.content());
    println!("Content: {}", post.content());
}

#[test]
fn f_test_v1() {
    let post = Post::new();

    assert_eq!("", post.content());
}

#[test]
fn f_test_v2() {
    let mut post = Post::new();

    post.add_text("Hello World");
    assert_eq!("", post.content());
}

#[test]
fn f_test_v3() {
    let mut post = Post::new();

    post.add_text("Hello World");
    post.request_review();
    assert_eq!("", post.content());
}

#[test]
fn f_test_v4_1() {
    let mut post = Post::new();

    post.add_text("Hello World");
    post.request_review();
    post.approve();
    assert_eq!("Hello World", post.content());
}

#[test]
fn f_test_v4_2() {
    let mut post = Post::new();

    post.add_text("Hello World");
    post.request_review();
    post.reject();
    assert_eq!("", post.content());
}
