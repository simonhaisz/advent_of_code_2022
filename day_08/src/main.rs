mod tree_grid;

use std::fs;

use tree_grid::TreeGrid;
use util::Timer;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();
    
    let data = fs::read_to_string("./day_08/input.txt")?;

    let tree_grid = TreeGrid::from_grid(&data);

    run_part_2(&tree_grid);

    Ok(())
}

#[allow(dead_code)]
fn run_part_1(tree_grid: &TreeGrid) {
    let visible_tree_count = tree_grid.visible_trees_count();

    println!("There are {} visible trees", visible_tree_count);
}

fn run_part_2(tree_grid: &TreeGrid) {
    let most_scenic = tree_grid.find_most_scenic_tree();

    println!("The most scenic tree has a score of {}", most_scenic.score());
}