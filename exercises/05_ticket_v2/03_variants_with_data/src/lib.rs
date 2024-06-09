#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
}

#[derive(Debug, PartialEq)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn assigned_to(&self) -> &str {
        match &self.status {
            Status::InProgress { assigned_to } => assigned_to,
            _ => panic!("Only In-Progress tickets can be assigned to someone"),
        }
    }

    pub fn assigned_to2(&self) -> &str {
        if let Status::InProgress { assigned_to } = &self.status { assigned_to } else { panic!("Only In-Progress tickets can be assigned to someone") }
    }

    pub fn assigned_to3(&self) -> &str {
        let Status::InProgress { assigned_to } = &self.status else { panic!("Only In-Progress tickets can be assigned to someone") }; assigned_to
    }

    pub fn assign_to(&mut self, person: String) {
        match &self.status {
            Status::ToDo => {
                self.status = Status::InProgress { assigned_to: person };
            }
            _ => panic!("Can only assign to `ToDo` tickets"),
        }
    }

    pub fn mark_as_done(&mut self) {
        match &self.status {
            Status::InProgress { .. } => {
                self.status = Status::Done;
            }
            _ => panic!("Can only mark `InProgress` tickets as done"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Only In-Progress tickets can be assigned to someone")]
    fn test_todo() {
        let ticket = Ticket::new(valid_title(), valid_description(), Status::ToDo);
        ticket.assigned_to();
    }

    #[test]
    #[should_panic(expected = "Only In-Progress tickets can be assigned to someone")]
    fn test_done() {
        let ticket = Ticket::new(valid_title(), valid_description(), Status::Done);
        ticket.assigned_to();
    }

    #[test]
    fn test_in_progress() {
        let ticket = Ticket::new(
            valid_title(),
            valid_description(),
            Status::InProgress {
                assigned_to: "Alice".to_string(),
            },
        );
        assert_eq!(ticket.assigned_to(), "Alice");
    }

    #[test]
    fn test_assign_to() {
        let person = Person { name: "Alice".to_string() };
        let mut ticket = Ticket::new(valid_title(), valid_description(), Status::ToDo);
        ticket.assign_to(person.name);
        assert_eq!(ticket.status, Status::InProgress { assigned_to: "Alice".to_string() });
    }

    #[test]
    #[should_panic(expected = "Can only assign to `ToDo` tickets")]
    fn test_assign_to_in_progress() {
        let person = Person { name: "Alice".to_string() };
        let mut ticket = Ticket::new(
            valid_title(),
            valid_description(),
            Status::InProgress {
                assigned_to: "Alice".to_string(),
            },
        );
        ticket.assign_to(person.name);
    }

    #[test]
    fn test_mark_as_done() {
        let mut ticket = Ticket::new(
            valid_title(),
            valid_description(),
            Status::InProgress {
                assigned_to: "Alice".to_string(),
            },
        );
        ticket.mark_as_done();
        assert_eq!(ticket.status, Status::Done);
    }

    #[test]
    #[should_panic(expected = "Can only mark `InProgress` tickets as done")]
    fn test_mark_as_done_todo() {
        let mut ticket = Ticket::new(valid_title(), valid_description(), Status::ToDo);
        ticket.mark_as_done();
    }
}
