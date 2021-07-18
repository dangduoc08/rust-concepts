pub trait State {
  fn content<'a>(&self, _post: &'a Post) -> &'a str {
    ""
  }
}

pub struct Post {
  pub state: Option<Box<dyn State>>,
  pub content: String,
}

impl Post {
  pub fn new() -> Post {
    Post {
      content: String::new(),
      state: Some(Box::new(Draft {})),
    }
  }

  pub fn write(&mut self, content: &str) {
    self.content.push_str(content);
  }

  pub fn set_state(&mut self, state: Box<dyn State>) -> &Self {
    self.state = Some(state);
    self
  }

  pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(self)
  }
}

pub struct Draft {}

impl State for Draft {}

pub struct Review {}

impl State for Review {}

pub struct Public {}

impl State for Public {
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    post.content.as_str()
  }
}

pub fn run() {
  let mut bbc_news = Post::new();
  bbc_news.write("Riot games sell to Tencent");
  assert_eq!("", bbc_news.content());

  bbc_news.set_state(Box::new(Review {}));
  assert_eq!("", bbc_news.content());

  bbc_news.set_state(Box::new(Public {}));
  assert_eq!("Riot games sell to Tencent", bbc_news.content());
}
