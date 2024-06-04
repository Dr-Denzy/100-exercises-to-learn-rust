// TODO: Add &mut-setters to the `Ticket` struct for each of its fields.
//   Make sure to enforce the same validation rules you have in `Ticket::new`!
//   Even better, extract that logic and reuse it in both places. You can use
//   private functions or private static methods for that.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
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

    fn allowed_status(status: &str) -> bool {
        match status {
            "To-Do" | "In Progress" | "Done" => true,
            _ => false,
        }
    }
    pub fn new(title: String, description: String, status: String) -> Ticket {
        // if title.is_empty() {
        //     panic!("Title cannot be empty");
        // }
        // if title.len() > 50 {
        //     panic!("Title cannot be longer than 50 bytes");
        // }
        // if description.is_empty() {
        //     panic!("Description cannot be empty");
        // }
        // if description.len() > 500 {
        //     panic!("Description cannot be longer than 500 bytes");
        // }
        // if status != "To-Do" && status != "In Progress" && status != "Done" {
        //     panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        // }

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


        Ticket {
            title,
            description,
            status,
        }
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


    // &mut setters
    pub fn set_title1(&mut self, new_title: String) {
        if !Self::valid_title(&new_title) {
            panic!("Title cannot be empty");
        }

        if Self::overly_long_title(&new_title) {
            panic!("Title cannot be longer than 50 bytes");
        }

        self.title = new_title;
    }


    pub fn set_description1(&mut self, new_descr: String) {
        if !Self::valid_title(&new_descr) {
            panic!("Description cannot be empty");
        }
        if Self::overly_long_description(&new_descr) {
            panic!("Description cannot be longer than 500 bytes");
        }
        self.description = new_descr;
    }

    pub fn set_status1(&mut self, new_status: String) {
        if !Self::allowed_status(&new_status) {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }
        self.status = new_status;
    }

    // mut setters
    pub fn set_title2(mut self, new_title: String) -> Self {
        if !Self::valid_title(&new_title) {
            panic!("Title cannot be empty");
        }

        if Self::overly_long_title(&new_title) {
            panic!("Title cannot be longer than 50 bytes");
        }

        self.title = new_title;
        self
    }


    pub fn set_description2(mut self, new_descr: String) -> Self {
        if !Self::valid_title(&new_descr) {
            panic!("Description cannot be empty");
        }
        if Self::overly_long_description(&new_descr) {
            panic!("Description cannot be longer than 500 bytes");
        }
        self.description = new_descr;
        self
    }

    pub fn set_status2(mut self, new_status: String) -> Self {
        if !Self::allowed_status(&new_status) {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }
        self.status = new_status;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn works() {

        // &mut setter option
        let mut ticket1 = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        ticket1.set_title1("A new title".into());
        ticket1.set_description1("A new description".into());
        ticket1.set_status1("Done".into());

        // mut setter option
        let ticket2 = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        let ticket2 = ticket2
            .set_title2("A new title".into())
            .set_description2("A new description".into())
            .set_status2("Done".into());

        assert_eq!(ticket2.title(), "A new title");
        assert_eq!(ticket2.description(), "A new description");
        assert_eq!(ticket2.status(), "Done");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title1("".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description1("".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_title1(overly_long_title());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_description1(overly_long_description());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status1("Funny".into());
    }
}
