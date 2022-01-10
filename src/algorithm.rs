use crate::permutation::Permutation;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compose() {
        assert_eq!(Algorithm::identity().compose("R", &Permutation::new(vec![vec![1, 2]])), Algorithm { permutation: Permutation::new(vec![vec![1, 2]]), moves: "R".to_owned() });
        assert_eq!(Algorithm { permutation: Permutation::new(vec![vec![1, 3]]), moves: "L".to_owned() }.compose("R", &Permutation::new(vec![vec![1, 2]])), Algorithm { permutation: Permutation::new(vec![vec![1, 3, 2]]), moves: "L R".to_owned() });
    }
}
