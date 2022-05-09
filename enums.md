## Enums
One word title inspired by "Boundaries", Ruby Conf 2012 talk Gary Bernhardt
<!-- .element: class="fragment" -->
---
## Enumerated type
_From Wikipedia_

In computer programming, an **enumerated type** (also called **enumeration**, **enum**, [...]) is a data type consisting of a set of named values called elements, members [...].
---
## Enum types in languages

Rust

```rust
#[derive(Debug)]
enum PostStatuses { Draft, Published, Removed }

fn debug_status(status: PostStatuses) {
    println!("{:?}", status);
}

fn main() {
    let status: PostStatuses = PostStatuses::Draft;
    debug_status(status);
    debug_status("Other"); // expected enum `PostStatuses`
    status = 1; // => expected enum `PostStatuses`
}
```
---
## Enum types in languages

Rust

```rust
#[derive(Debug)]
enum PostStatuses { Draft, Published, Removed }

fn main() {
    let status: PostStatuses = PostStatuses::Draft;

    match status {
        PostStatuses::Draft => { /* ... */ }
        PostStatuses::Published => { /* ... */ }
    }
    // error[E0004]: non-exhaustive patterns: `Removed`
    // not covered
}
```
---
## Enums in Rails
Rails introduced enum in version 4.1. It’s a handy feature, which lets you describe any trait of an object in a very friendly and human readable way. If you need to keep track of the state of an entity, you will most likely add a field to the model and store a value there.
---
## Rails Interface

```ruby
class Post < ApplicationRecord
  enum :status, [:draft, :published, :removed]
end

Post.statuses
# => {"draft"=>0, "published"=>1, "removed"=>2}
```
---
## Rails Interface

```ruby
post = Post.new(status: :draft) # => <Post...>
post.draft? # => true
post.not_published? # => true
post.removed? # => false
post.status # => "draft"
```
<!-- .element: class="fragment" -->

```ruby
Post.draft
# SELECT "posts".* FROM "posts" WHERE "posts"."status" = 0
# All Posts with status "draft"

Post.not_published
# SELECT "posts".* FROM "posts" WHERE "posts"."status" != 1
# All Posts with status other than "published"

post = Post.draft.new
post.draft? # => true
```
<!-- .element: class="fragment" -->

```ruby
post.published!
# UPDATE "posts" SET "status" = ?, "updated_at" = ?
# WHERE "post"."id" = ? [["status", 1],
# ["updated_at", "2021-05-18 16:13:08.524320"], ["id", 1]]
```
<!-- .element: class="fragment" -->
---
## Advanced Rails Interface

```ruby
class Post < ApplicationRecord
  enum(
    :status,
    [:draft, :published, :removed],
    default: :draft,
    scopes: false,
    prefix: "post",
    suffix: true,
  )
end
```
---

New syntax for `enum` in Rails 7.0 to avoid leading `_`
from reserved options.

Before:

```ruby
class Post < ActiveRecord::Base
  enum(
    status: [:draft, :published, :removed],
    _prefix: true,
    _scopes: false,
    _default: :draft,
   )
end
```

After:

```ruby
class Post < ActiveRecord::Base
  enum(
    :status,
    [:draft, :published, :removed],
    prefix: true,
    scopes: false,
    default: :draft,
   )
end
```