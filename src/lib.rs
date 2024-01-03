#[derive(Debug)]
pub enum GridParseError {
    GridEmpty,
    InconsistentLineLengths,
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub letters: Vec<char>,
    pub width: usize,
    pub height: usize,
}
impl Grid {
    pub fn from_chars(lines: Vec<Vec<char>>) -> Result<Grid, GridParseError> {
        use GridParseError as GPE;
        if lines.is_empty() {
            return Err(GPE::GridEmpty);
        }
        let width: usize = lines[0].len();
        let mut letters: Vec<char> = Vec::new();
        for l in lines {
            if l.len() != width {
                return Err(GPE::InconsistentLineLengths)
            }
            letters.extend(l.iter());
        }
        Ok(Grid {
            height: letters.len() / width,
            letters,
            width,
        })
    }
    pub fn get_coord(&self, index: usize) -> [usize; 2] {
        [index % self.width, index / self.width]
    }
    pub fn get_index(&self, coordinate: [usize; 2]) -> usize {
        coordinate[0] + self.width * coordinate[1]
    }

    pub fn in_bounds_coord(&self, [a, b]: [isize; 2]) -> bool {
        a >= 0 && b >= 0 && a < self.width as isize && b < self.height as isize
    }

    pub fn neighbours(&self, index: usize) -> Vec<usize> {
        let [x, y] = self.get_coord(index).map(|n| n as isize);
        let neighbours: [[isize; 2]; 8] = [
            [x + 1, y],
            [x + 1, y + 1],
            [x, y + 1],
            [x - 1, y + 1],
            [x - 1, y],
            [x - 1, y - 1],
            [x, y - 1],
            [x + 1, y - 1],
        ];
        neighbours.iter()
            .filter(|&&neighbour| self.in_bounds_coord(neighbour))
            .map(|&coord| self.get_index(coord.map(|i| i as usize)))
            .collect()
    }
    pub fn find_word(&self, word: &[char]) -> Option<[usize; 2]> {
        for i in 0..(self.letters.len()) {
            // The tuple represents the index to check and which letter in the word to check from.
            let mut to_check_stack: Vec<(usize, usize)> = vec![(i, 0)];
            while !to_check_stack.is_empty() {
                let next = to_check_stack.pop().unwrap();
                if next.1 == word.len() {
                    return Some(self.get_coord(i))
                }
                if self.letters[next.0] == word[next.1] {
                    to_check_stack.extend(self
                        .neighbours(next.0)
                        .iter()
                        .map(|&n_index| (n_index, next.1 + 1))
                    );
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        // Taken from Undertale.
        let grid_txt = String::from(
            "GIASFCLFUBREHBER
NPBAVUUJJCSEOMEO
IWLSNOTELEKSTMFB
RLXETMONSTERMNGO
PMDIAMREMAUUJHIT
SCIGARSVXRSOUDCW
"
        );
        let words_txt = String::from(
"FALL
WINTER
SPRING
SUMMER
MONSTER
SKELETONS
MERMAID
ROBOT
CIGARS
CIG
GIASFCLFUBREHBER
HOT
"
        );
        let grid_chars: Vec<Vec<char>> = grid_txt.lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();
        let grid = Grid::from_chars(grid_chars).unwrap();
        let coords: Vec<Option<[usize; 2]>> = words_txt
            .trim()
            .lines()
            .map(|w| grid.find_word(&w.chars().collect::<Vec<char>>()[..]))
            .collect();
        let expected = vec![
            Some([4, 0]),
            Some([15, 5]),
            Some([0, 5]),
            Some([10, 5]),
            Some([5, 3]),
            Some([10, 1]),
            Some([8, 4]),
            Some([15, 0]),
            Some([1, 5]),
            Some([1, 5]),
            Some([0, 0]),
            Some([12, 0]),
        ];
        assert_eq!(expected, coords)
    }
}
