mod ticket {
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
    }
}

// TODO: **Exceptionally**, you'll be modifying both the `ticket` module and the `tests` module
//  in this exercise.
#[cfg(test)]
mod tests {
    // TODO: Add the necessary `pub` modifiers in the parent module to remove the compiler
    //  errors about the use statement below.
    use super::ticket::Ticket;

    // Be careful though! We don't want this function to compile after you have changed
    // visibility to make the use statement compile!
    // Once you have verified that it indeed doesn't compile, comment it out.
    fn should_not_be_possible() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

        // You should be seeing this error when trying to run this exercise:
        //
        // error[E0616]: field `description` of struct `encapsulation::ticket::Ticket` is private
        //    |
        //    |              assert_eq!(ticket.description, "A description");
        //    |                         ^^^^^^^^^^^^^^^^^^
        //
        // TODO: Once you have verified that the below does not compile,
        //   comment the line out to move on to the next exercise!
        // assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
        // This should be impossible as well, with a similar error as the one encountered above.
        // (It will throw a compilation error only after you have commented the faulty line
        // in the previous test - next compilation stage!)
        //
        // This proves that `Ticket::new` is now the only way to get a `Ticket` instance.
        // It's impossible to create a ticket with an illegal title or description!
        //
        // TODO: Once you have verified that the below does not compile,
        //   comment the lines out to move on to the next exercise!
        // let ticket = Ticket {
        //     title: "A title".into(),
        //     description: "A description".into(),
        //     status: "To-Do".into(),
        // };
    }
}
