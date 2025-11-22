// another way to doing it;

pub struct Post {
    content: String,
}

impl Post {
    pub fn new(content: &str) -> DraftPost {
        DraftPost {
            constent: String::from(content),
        }
    }
    pub fn content(&self) -> &str {
        self.content.as_ref()
    }
}

pub struct DraftPost {
    constent: String,
}

impl DraftPost {
    pub fn request_review(self) -> PendingReview {
        PendingReview {
            content: self.constent,
        }
    }
    pub fn add_text(&mut self, cont: &str) {
        self.constent.push_str(cont);
    }
}
pub struct PendingReview {
    content: String,
}
impl PendingReview {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
// one way doing it
// pub struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
// }
//
// impl Post {
//     pub fn new() -> Self {
//         Self {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }
//     pub fn content(&self) -> &str {
//         self.state.as_ref().unwrap().content(self)
//     }
//
//     pub fn add_text(&mut self, text: &str) {
//         self.content
//             .push_str(self.state.as_ref().unwrap().add_text(text));
//     }
//     pub fn request_review(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review());
//         }
//     }
//     pub fn reject(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.reject());
//         }
//     }
//
//     pub fn approve(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve());
//         }
//     }
// }
// trait State {
//     fn add_text<'a>(&self, _: &'a str) -> &'a str {
//         ""
//     }
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     fn content<'a>(&self, _: &'a Post) -> &'a str {
//         ""
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State>;
// }
//
// struct Draft {}
// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn add_text<'a>(&self, content: &'a str) -> &'a str {
//         content
//     }
// }
//
// struct PendingReview {}
// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Draft {})
//     }
// }
//
// struct Published {}
// impl State for Published {
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         post.content.as_str()
//     }
// }
