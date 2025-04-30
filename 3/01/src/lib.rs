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
    pub fn is_open(&self) -> bool {
        self.status == "Open"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_open() {
        let ticket = Ticket {
            title: "Build a ticket system".into(),
            description: "A Kanban board".into(),
            status: "Open".into(),
        };

        assert_eq!(true, ticket.is_open());

        let ticket_closed = Ticket {
            title: "Build a ticket system".into(),
            description: "A Kanban board".into(),
            status: "Closed".into(),
        };
        assert_ne!(true, ticket_closed.is_open());
    }
}
