pub struct TreeGrid {
    trees: Vec<u8>,
    rows: i32,
    columns: i32,
}

impl TreeGrid {
    pub fn from_grid(text: &str) -> Self {
        let lines = text
            .split("\n")
            .collect::<Vec<&str>>();

        TreeGrid::from_lines(&lines)
    }

    pub fn from_lines(lines: &[&str]) -> Self {
        let mut trees = vec![];

        let mut rows = 0;
        let mut columns = -1;
        for line in lines.iter() {
            let char_count = line.chars().count() as i32;
            if char_count == 0 {
                continue;
            }
            if columns == -1 {
                columns = char_count;
            } else {
                assert_eq!(columns, char_count);
            }
            rows += 1;
            for c in line.chars() {
                assert!(c.is_numeric(), "only numeric characters should be included - {}", c);
                trees.push(c.to_digit(10).unwrap() as u8);
            }
        }

        Self { trees, rows, columns }
    }

    fn grid_ref(&self, index: usize) -> (i32, i32) {
        let row = (index as i32 / self.columns) as i32;
        let column = (index as i32 % self.columns) as i32;

        (row, column)
    }

    fn index(&self, row: i32, column: i32) -> usize {
        row as usize * self.columns as usize + column as usize
    }

    fn next(&self, index: usize, direction: &SearchDirection) -> Option<usize> {
        let (row, column) = self.grid_ref(index);
        let (row, column) = match direction {
            SearchDirection::Up => (row.checked_sub(1).unwrap(), column),
            SearchDirection::Down => (row.checked_add(1).unwrap(), column),
            SearchDirection::Left => (row, column.checked_sub(1).unwrap()),
            SearchDirection::Right => (row, column.checked_add(1).unwrap()),
        };
        if row < 0 || row >= self.rows as i32 || column < 0 || column >= self.columns as i32 {
            None
        } else {
            Some(self.index(row, column))
        }
    }

    pub fn visible_trees_count(&self) -> u64 {
        let mut visible_count = 0;

        for index in 0..self.trees.len() {
            if search_up(self, index).1 || search_down(self, index).1 || search_left(self, index).1 || search_right(self, index).1 {
                visible_count += 1
            }
        }

        visible_count
    }

    pub fn find_most_scenic_tree(&self) -> ScenicInfo {
        let mut most_scenic_tree: Option<ScenicInfo> = None;

        for index in 0..self.trees.len() {
            let up = search_up(self, index).0;
            let down = search_down(self, index).0;
            let left = search_left(self, index).0;
            let right = search_right(self, index).0;

            let score = up * down * left * right;

            let replace = if let Some(most_scenic) = &most_scenic_tree {
                score > most_scenic.score
            } else {
                true
            };
            if replace {
                most_scenic_tree.replace(ScenicInfo { index, score });
            }
        }

        most_scenic_tree.unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ScenicInfo {
    index: usize,
    score: u64,
}

impl ScenicInfo {
    pub fn score(&self) -> u64 {
        self.score
    }
}

pub enum SearchDirection {
    Up,
    Down,
    Left,
    Right
}

fn search_up(tree_grid: &TreeGrid, tree: usize) -> (u64, bool) {
    search(tree_grid, tree, SearchDirection::Up)
}

fn search_down(tree_grid: &TreeGrid, tree: usize) -> (u64, bool) {
    search(tree_grid, tree, SearchDirection::Down)
}

fn search_left(tree_grid: &TreeGrid, tree: usize) -> (u64, bool) {
    search(tree_grid, tree, SearchDirection::Left)
}

fn search_right(tree_grid: &TreeGrid, tree: usize) -> (u64, bool) {
    search(tree_grid, tree, SearchDirection::Right)
}

fn search(tree_grid: &TreeGrid, origin: usize, direction: SearchDirection) -> (u64, bool) {
    let size = *tree_grid.trees.get(origin).unwrap();

    let mut visible_tree_count = 0;
    
    let mut current = origin;
    while let Some(next) = tree_grid.next(current, &direction) {
        visible_tree_count += 1;
        let next_size = *tree_grid.trees.get(next).expect(&format!("There should be a tree with index {}", next));
        if next_size >= size {
            return (visible_tree_count, false);
        }
        current = next;
    }

    (visible_tree_count, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        let tree_grid = TreeGrid::from_grid(
r#"
30373
25512
65332
33549
35390
"#
        );

        assert_eq!(25, tree_grid.trees.len());
        assert_eq!(3, tree_grid.trees[0]);
        assert_eq!(3, tree_grid.trees[12]);
        assert_eq!(0, tree_grid.trees[24]);
    }

    #[test]
    fn visibility() {
        let tree_grid = TreeGrid::from_grid(
r#"
30373
25512
65332
33549
35390
"#
        );

        assert_eq!(21, tree_grid.visible_trees_count());
    }

    #[test]
    fn most_scenic() {
        let tree_grid = TreeGrid::from_grid(
r#"
30373
25512
65332
33549
35390
"#
        );

        assert_eq!(ScenicInfo { index: 17, score: 8 }, tree_grid.find_most_scenic_tree());
    }
}