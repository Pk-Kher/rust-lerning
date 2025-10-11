pub trait Messenger {
    fn send(&self, message: &str);
}
pub struct LimitTracker<T: Messenger> {
    messenger: T,
    value: usize,
    max: usize,
}

impl<T> LimitTracker<T>
where
    T: Messenger,
{
    fn new(messanger: T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger: messanger,
            value: 0,
            max: max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_usage = value as f32 / self.max as f32;
        if percentage_usage >= 1.0 {
            self.messenger.send("you hit the message limit");
        } else if percentage_usage >= 0.9 {
            self.messenger.send("you hit the 90% limit");
        } else if percentage_usage >= 0.7 {
            self.messenger.send("you hit the 70% limit");
        }
    }
}
pub fn start_fn() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMessenger {
        sent_message: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_message: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_message.borrow_mut().push(String::from(message));
            // self.sent_message.push(String::from(message)); // without refcell
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messager = MockMessenger::new();
        mock_messager.send("helloo");
        // let data = RefCell::new(10);
        // let m1 = data.borrow_mut();
        // let m2 = data.borrow_mut();
        // *m1=20;
        println!();
        assert_eq!(mock_messager.sent_message.borrow().len(), 1);
    }
}
