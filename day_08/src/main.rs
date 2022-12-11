mod tree_grid;

use std::fs;

use tree_grid::TreeGrid;
use util::Timer;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();
    
    let data = fs::read_to_string("./day_08/input.txt")?;

    let tree_grid = TreeGrid::from_grid(&data);

    let visible_tree_count = tree_grid.visible_trees_count();

    println!("There are {} visible trees", visible_tree_count);

    Ok(())
}
