pub mod ticket {
    use std::thread::sleep;

    pub struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        pub fn new(title: String, description: String, status: String) -> Self {
            if !Self::valid_title(&title) {
                panic!("Title cannot be empty")
            }

            if Self::overly_long_title(&title) {
                panic!("Title cannot be longer than 50 bytes")
            }

            if !Self::valid_description(&description) {
                panic!("Description cannot be empty")
            }

            if Self::overly_long_description(&description) {
                panic!("Description cannot be longer than 500 bytes")
            }

            if !Self::allowed_status(&status) {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed")
            }


            Self {
                title,
                description,
                status,
            }
        }

        fn allowed_status(status: &str) -> bool {
            match status {
                "To-Do" | "In Progress" | "Done" => true,
                _ => false,
            }
        }

        fn valid_title(title: &str) -> bool {
            title.len() > 0
        }

        fn overly_long_title(title: &str) -> bool {
            title.as_bytes().len() > 50
        }

        fn valid_description(descr: &str) -> bool {
            descr.len() > 0
        }

        fn overly_long_description(descr: &str) -> bool {
            descr.as_bytes().len() > 500
        }

        // TODO: Add three public methods to the `Ticket` struct:
        //  - `title` that returns the `title` field.
        //  - `description` that returns the `description` field.
        //  - `status` that returns the `status` field.

        pub fn title(self) -> String {
            self.title
        }

        pub fn description(self) -> String {
            self.description
        }

        pub fn status(self) -> String {
            self.status
        }
    }
}


#[cfg(test)]
mod tests {
    use super::ticket::Ticket;

    #[test]
    fn description() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.description(), "A description");
    }

    #[test]
    fn title() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.title(), "A title");
    }

    #[test]
    fn status() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.status(), "To-Do");
    }
}

