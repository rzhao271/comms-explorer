use std::fmt;
use std::hash::{Hash, Hasher};

use crate::permutation::Permutation;

#[derive(Clone, Debug)]
pub struct Algorithm {
    pub permutation: Permutation,
    pub moves: String
}

impl Algorithm {
    pub fn identity() -> Algorithm {
        Algorithm {
            permutation: Permutation::new(vec![]),
            moves: "".to_owned()
        }
    }

    pub fn compose(&self, other_moves: &str, other_permutation: &Permutation) -> Algorithm {
        Algorithm {
            permutation: self.permutation.compose(other_permutation),
            moves: if self.moves.len() == 0 {
                other_moves.to_owned()
            }
            else {
                format!("{} {}", &self.moves, other_moves)
            }
        }
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.permutation, self.moves)
    }
}

impl PartialEq for Algorithm {
    fn eq(&self, other: &Self) -> bool {
        self.permutation == other.permutation
    }
}

impl Eq for Algorithm {
}

impl Hash for Algorithm {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.permutation.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compose() {
        assert_eq!(Algorithm::identity().compose("R", &Permutation::new(vec![vec![1, 2]])),
            Algorithm { permutation: Permutation::new(vec![vec![1, 2]]), moves: "R".to_owned() });
        assert_eq!(Algorithm { permutation: Permutation::new(vec![vec![1, 3]]), moves: "L".to_owned() }.compose("R", &Permutation::new(vec![vec![1, 2]])),
            Algorithm { permutation: Permutation::new(vec![vec![1, 3, 2]]), moves: "L R".to_owned() });
    }

    #[test]
    fn should_display() {
        assert_eq!(Algorithm::identity().to_string(), "[id] ");
        assert_eq!(Algorithm { permutation: Permutation::new(vec![vec![1, 2]]), moves: "R".to_owned() }.to_string(),
            "(1 2) R");
    }

    #[test]
    fn should_eq() {
        assert_eq!(Algorithm::identity() == Algorithm::identity(), true);
        assert_eq!(Algorithm::identity() != Algorithm::identity(), false);
        assert_eq!(Algorithm { permutation: Permutation::new(vec![vec![1, 2]]), moves: "R".to_owned() } == Algorithm { permutation: Permutation::new(vec![vec![1, 2]]), moves: "R L L L R R R".to_owned() }, true);
        assert_eq!(Algorithm { permutation: Permutation::new(vec![vec![1, 2]]), moves: "R".to_owned() } == Algorithm { permutation: Permutation::new(vec![vec![1, 3]]), moves: "R".to_owned() }, false);
    }
}
