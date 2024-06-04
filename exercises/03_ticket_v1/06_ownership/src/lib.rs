// TODO: based on what we just learned about ownership, it sounds like immutable references
//   are a good fit for our accessor methods.
//   Change the existing implementation of `Ticket`'s accessor methods take a reference
//   to `self` as an argument, rather than taking ownership of it.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
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

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;

    #[test]
    fn works() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        // If you change the signatures as requested, this should compile:
        // we can call these methods one after the other because they borrow `self`
        // rather than taking ownership of it.
        assert_eq!(ticket.title(), "A title");
        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.status(), "To-Do");
    }
}
