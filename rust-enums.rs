#[derive(Debug)]
#[derive(PartialEq)]
enum PostStatuses { Draft, Published, Removed }

fn debug_status(status: PostStatuses) {
    println!("{:?}", status);
}

fn main() {
    let status: PostStatuses = PostStatuses::Draft;

    if status == "Other" { 
        println!("XX");
    }

    debug_status(status); // expected enum `PostStatuses`
    debug_status("Draft"); // => expected enum `PostStatuses`
    status = "Other";
}