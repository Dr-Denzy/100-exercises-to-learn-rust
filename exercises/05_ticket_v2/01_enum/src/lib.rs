// TODO: use `Status` as type for `Ticket::status`
//   Adjust the signature and implementation of all other methods as necessary.

#[derive(Debug, PartialEq)]
// `derive`s are recursive: it can only derive `PartialEq` if all fields also implement `PartialEq`.
// Same holds for `Debug`. Do what you must with `Status` to make this work.
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
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

    // fn allowed_status(status: &Status) -> bool {
    //     match status {
    //         Status::ToDo | Status::InProgress | Status::Done => true,
    //         _ => false,
    //     }
    // }

    pub fn new(title: String, description: String, status: Status) -> Ticket {
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

    pub fn status(&self) -> &Status {
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

    pub fn set_status1(&mut self, new_status: Status) {
        // if !Self::allowed_status(&new_status) {
        //     panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        // }
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

    pub fn set_status2(mut self, new_status: Status) -> Self {
        self.status = new_status;
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};

    #[test]
    fn test_partial_eq() {
        let title = valid_title();
        let description = valid_description();
        let ticket1 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::ToDo,
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::ToDo,
        };
        assert_eq!(ticket1, ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let title = valid_title();
        let status = Status::ToDo;
        let ticket1 = Ticket {
            title: title.clone(),
            description: "description".to_string(),
            status,
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: "description2".to_string(),
            status,
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let description = valid_description();
        let status = Status::InProgress;
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: description.clone(),
            status,
        };
        let ticket2 = Ticket {
            title: "title2".to_string(),
            description: description.clone(),
            status,
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let title = valid_title();
        let description = valid_description();
        let ticket1 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::InProgress,
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::Done,
        };
        assert_ne!(ticket1, ticket2);
    }
}
