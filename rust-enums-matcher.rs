#[derive(Debug)]
enum PostStatuses { Draft, Published, Removed }

fn main() {
    let status: PostStatuses = PostStatuses::Draft;

    match status {
        PostStatuses::Draft => { /* ... */ }
        PostStatuses::Published => { /* ... */ }
    } // error[E0004]: non-exhaustive patterns: `Removed` not covered
}