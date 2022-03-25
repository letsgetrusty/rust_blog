use rust_blog::{User, Post};

fn main() {
    let user1 = User::new("testuser1234".to_owned())
                        .unwrap_or_default();

    println!("{:?}", user1);

    let post1 = Post::default();

    let post2 = Post::new("example content".to_owned());

    println!("{:?}", post1);
    println!("{:?}", post2);
}
