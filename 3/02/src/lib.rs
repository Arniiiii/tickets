pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

pub struct Configuration {
    pub version: u32,
    pub active: bool,
}

impl Ticket {
    fn validate(title: &String, description: &String, status: &String) -> Result<(), &'static str> {
        if title.len() == 0 {
            return Err("Title cannot be empty");
        }

        if description.len() == 0 {
            return Err("Description cannot be empty");
        }

        if title.as_bytes().len() > 50 {
            return Err("Title cannot be longer than 50 bytes");
        }

        if description.as_bytes().len() > 500 {
            return Err("Description cannot be longer than 500 bytes");
        }

        let valid_statuses = ["To-Do", "In Progress","Done"];
        if ! valid_statuses.contains(&status.as_str()) {
            return Err("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ok(())
    }

    pub fn new(title: String, description: String, status: String) -> Ticket {
        Ticket::validate(&title, &description, &status).unwrap_or_else(|error| {
            panic!("{error}");
        });

        Self {
            title,
            description,
            status,
        }
    }

    pub fn is_open(&self) -> bool {
        self.status == "Open"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
