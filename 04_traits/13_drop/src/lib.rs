// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

#[derive(Debug)]
struct DropBomb {
    is_ticking: bool,
}

impl DropBomb {
    fn new() -> Self {
        Self { is_ticking: true }
    }
    fn defuse(&mut self) {
        self.is_ticking = false;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if self.is_ticking {
            panic!("OOOOOOOO THE DROP BOMB HAS EXPLODED!!!! U R PWNED")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
