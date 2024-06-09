// TODO: Convert the `Ticket::new` method to return a `Result` instead of panicking.
//   Use `String` as the error type.
//   Note: I used a custom enum type (TicketError) instead.

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

#[derive(Debug, PartialEq)]
enum TicketError {
    EmptyTitle,
    TitleTooLong,
    EmptyDescription,
    DescriptionTooLong,
}

impl TicketError {
    fn message(&self) -> &str {
        match self {
            TicketError::EmptyTitle => "Title cannot be empty",
            TicketError::TitleTooLong => "Title cannot be longer than 50 bytes",
            TicketError::EmptyDescription => "Description cannot be empty",
            TicketError::DescriptionTooLong => "Description cannot be longer than 500 bytes",
        }
    }
}


impl Ticket {
    fn valid_title(title: &str) -> bool {
        !title.is_empty()
    }

    fn overly_long_title(title: &str) -> bool {
        title.as_bytes().len() > 50
    }

    fn valid_description(descr: &str) -> bool {
        !descr.is_empty()
    }

    fn overly_long_description(descr: &str) -> bool {
        descr.as_bytes().len() > 500
    }


    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, TicketError> {
        if !Self::valid_title(&title) {
            return Err(TicketError::EmptyTitle);
        }

        if Self::overly_long_title(&title) {
            return Err(TicketError::TitleTooLong);
        }

        if !Self::valid_description(&description) {
            return Err(TicketError::EmptyDescription);
        }

        if Self::overly_long_description(&description) {
            return Err(TicketError::DescriptionTooLong);
        }


        Ok(Ticket {
            title,
            description,
            status,
        })
    }

    // Assessors
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
    pub fn set_title(&mut self, new_title: String) -> Result<(), TicketError> {
        if !Self::valid_title(&new_title) {
            return Err(TicketError::EmptyTitle);
        }

        if Self::overly_long_title(&new_title) {
            return Err(TicketError::TitleTooLong);
        }

        self.title = new_title;
        Ok(())
    }


    pub fn set_description(&mut self, new_descr: String) -> Result<(), TicketError> {
        if !Self::valid_title(&new_descr) {
            return Err(TicketError::EmptyDescription);
        }
        if Self::overly_long_description(&new_descr) {
            return Err(TicketError::DescriptionTooLong);
        }
        self.description = new_descr;
        Ok(())
    }

    pub fn set_status(&mut self, new_status: Status) {
        self.status = new_status;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn title_cannot_be_empty() {
        let error = Ticket::new("".into(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(error, TicketError::EmptyTitle);
    }

    #[test]
    fn description_cannot_be_empty() {
        let error = Ticket::new(valid_title(), "".into(), Status::ToDo).unwrap_err();
        assert_eq!(error, TicketError::EmptyDescription);
    }

    #[test]
    fn title_cannot_be_longer_than_fifty_chars() {
        let error =
            Ticket::new(overly_long_title(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(error, TicketError::TitleTooLong);
    }

    #[test]
    fn description_cannot_be_longer_than_500_chars() {
        let error =
            Ticket::new(valid_title(), overly_long_description(), Status::ToDo).unwrap_err();
        assert_eq!(error, TicketError::DescriptionTooLong);
    }
}
