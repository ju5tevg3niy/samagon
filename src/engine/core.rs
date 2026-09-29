#[derive(Debug)]
pub struct SmgnEngine {
    should_quit: bool,
}

impl Default for SmgnEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SmgnEngine {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn process_events<E: SmgnEventProvider>(&mut self, event_provider: &mut E) {
        while let Some(event) = event_provider.get_event() {
            match event {
                SmgnEvent::Quit => {
                    self.should_quit = true;
                }
            }
        }
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}

#[derive(Debug)]
pub enum SmgnEvent {
    Quit,
}

pub trait SmgnEventProvider {
    fn get_event(&mut self) -> Option<SmgnEvent>;
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::VecDeque;

    struct TestEventQueueProvider(VecDeque<SmgnEvent>);

    impl SmgnEventProvider for TestEventQueueProvider {
        fn get_event(&mut self) -> Option<SmgnEvent> {
            self.0.pop_front()
        }
    }

    #[test]
    fn trivial() {
        let _engine = SmgnEngine::new();
    }

    #[test]
    fn quit_event() {
        let mut engine = SmgnEngine::new();

        assert!(!engine.should_quit);

        let mut event_queue = TestEventQueueProvider([SmgnEvent::Quit].into());

        engine.process_events(&mut event_queue);

        assert!(engine.should_quit);
    }
}
