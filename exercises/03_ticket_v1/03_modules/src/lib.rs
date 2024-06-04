mod helpers {
    // TODO: Make this code compile, either by adding a `use` statement or by using
    //  the appropriate path to refer to the `Ticket` struct.


    use crate::Ticket;

    fn create_todo_ticket(title: String, description: String) -> Ticket {
        Ticket::new(title, description, "To-Do".into())
    }
}

struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    fn new(title: String, description: String, status: String) -> Self {
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

