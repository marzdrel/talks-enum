#[derive(Debug)]
enum PostStatuses { Draft, Published, Removed }

fn debug_status(status: PostStatuses) {
    println!("{:?}", status);
}

fn main() {
    let status: PostStatuses = PostStatuses::Draft;

    if PostStatuses::Draft > status {
        println!("XX");
    }

    debug_status(status); // expected enum `PostStatuses`
    debug_status("Other"); // => expected enum `PostStatuses`
    status = "Other";
}