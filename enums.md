## Enums
One word title inspired by "Boundaries", Ruby Conf 2012 talk by Gary Bernhardt
<!-- .element: class="fragment" -->

Notes:
  - This is about Rails enums, but I wanted one word title, so no "Rails Enums"
---
## Enumerated type
_From Wikipedia_

In computer programming, an **enumerated type** (also called **enumeration**, **enum**, [...]) is a data type consisting of a set of named values called elements, members [...].

Notes:
- Typ wyliczeniowy
- Typ składający się ze zbioru nazwanych wartości
- Miesiące: styczeń, luty...
---
## Native Enums in Ruby

`¯\_(ツ)_/¯`
<!-- .element: class="fragment" -->

Notes:
- Do we have native enums in Ruby in stdlib?
- module Enumerable, class Enumerator - nope
- Dynamic language
- No type sigs (now we have RBS, also sorbet)
- Not much of a point of having Enum type
---
## "Native" Enum types

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
    debug_status("Draft"); // expected enum `PostStatuses`
    status = 1; // => expected enum `PostStatuses`
    if status != "Draft" { /* ... */ }
    // => expected enum `PostStatuses`
}
```

Notes:
- Type inference in Rust
- Dynamic vs Static typing
- Strong vs Weak typing
- if status == "Other"
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

Notes:
- Why not case?
- Adding new status fails the matcher.
---
## Type checking in Ruby using RBS

```
class Enum
  # [...]
end

# RBS: def show: (Enum enum) -> nil
def show(enum)
  # [...]
end

show "String" # Ruby::ArgumentTypeMismatch
              # Cannot pass a value of type `::String`
              # as an argument of type `::Enum`
```
<!-- .element: class="fragment" -->

Notes:
- gem install rbs (RBS in Ruby 3.0.0)
- gem install steep (type checking)
- sorbet by Stripe
- Python typing in 3.5
---
## Enum in Ruby using external libraries

```ruby
require "dry-types"

PostStatuses =
  Dry::Types["string"].enum("draft", "published", "removed")

PostStatuses.values # => ["draft", "published", "removed"]

PostStatuses["draft"] # => "draft"
PostStatuses["other"] # => Dry::Types::ConstraintError

```
---
## Enums in Rails
Rails introduced enum in ActiveRecord in version 4.1. It’s a handy feature,
which lets you describe any trait of an object in a very friendly and human
readable way. If you need to keep track of the state of an entity, you will
most likely add a field to the model and store a value there.
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
post = Post.new(status: :draft) # => "#<Post id: nil, ...>"
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

post = Post.draft.new # => "#<Post id: nil, ...>"
post.draft? # => true
```
<!-- .element: class="fragment" -->

```ruby
post.published!
# UPDATE "posts" SET "status" = ?, "updated_at" = ?
# WHERE "post"."id" = ? [["status", 1],
# ["updated_at", "2022-05-18 16:13:08.524320"], ["id", 1]]
```
<!-- .element: class="fragment" -->
---
## "Type checking"
```ruby
Post.new(status: :other)
# assert_valid_value': 'other' is not
# a valid status (ArgumentError)
```


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

post = Post.new # => "#<Post id: nil, ...>"
post.post_draft_status? # => true
```
---

New syntax for `enum` in Rails 7.0 to avoid leading `_`
from reserved options.

```ruby
class Post < ApplicationRecord
  enum(
    status: [:draft, :published, :removed],
    _prefix: true,
    _scopes: false,
    _default: :draft,
  )
end
```

```ruby
class Post < ApplicationRecord
  enum(
    :status,
    [:draft, :published, :removed],
    prefix: true,
    scopes: false,
    default: :draft,
  )
end
```
---
### Underlying data type

By default Rails Enums use Integer type to store values.

```ruby
class Post < ApplicationRecord
  enum :status, [:draft, :published]
end

Post.statuses
# => {"draft"=>0, "published"=>1}
```

```ruby
class Post < ApplicationRecord
  enum :status, [:removed, :draft, :published]
end

Post.statuses
# => {"removed"=>0, "draft"=>1, "published"=>2}
```
<!-- .element: class="fragment" -->

```
Post.removed.to_sql
# => SELECT "posts".* FROM "posts" WHERE "posts"."status" = 0
```
<!-- .element: class="fragment" -->
---
### Explicit values for enum elements
Even though Array syntax seems more pure and compact you should really avoid it at all cost.

```ruby
class Post < ApplicationRecord
  enum status: { draft: 0, published: 1, removed: 2 }
end

Post.statuses
# => {"draft"=>0, "published"=>1, "removed"=>2}
```
---
### Integer values

```ruby
class AddStatusToPosts < ActiveRecord::Migration[7.0]
  def change
    add_column(
      :posts,
      :status,
      :integer,
      null: false,
      limit: 2,
    )
  end
end
```

Notes:
- "limit" defines size of int (shortint for 2)
- MySQL has tinyint (limit: 1)
---
### Integer values: Readability issues

```ruby
Post.removed.to_sql
# => SELECT "posts".* FROM "posts" WHERE "posts"."status" = 2
```
---
### Integer values: Readability issues

There are still many places in ActiveRecord syntax, where the automatic
translation between labels and values cannot be performed.

```ruby
User
  .joins(<<~SQL.squish)
    LEFT JOINS posts
      ON posts.user_id = user.id
        AND posts.status = 1
  SQL
```
<!-- .element: class="fragment" -->

```ruby
status = Post.statuses.fetch("published")

User
  .joins(format(<<~SQL.squish, status: status))
    LEFT JOINS posts
      ON posts.user_id = user.id
        AND posts.status = %<status>s
  SQL
```
<!-- .element: class="fragment" -->
---
### Storing values as Strings

```ruby
class AddStatusToPosts < ActiveRecord::Migration[7.0]
  def change
    add_column(
      :posts,
      :status,
      :string,
      null: false,
    )
  end
end
```
---
### Storing values as Strings

```ruby
class Post < ApplicationRecord
  enum :status, { draft: "draft", published: "published" }
end

Post.statuses
# => {"draft"=>"draft", "published"=>"published"}
```

```ruby
User
  .joins(<<~SQL.squish)
    LEFT JOINS posts
      ON posts.user_id = user.id
        AND posts.status = 'published'
  SQL
```
<!-- .element: class="fragment" -->

Notes:
- Strings consume more space
- Strings might have performance impact
- Strings don't prevent typo errors in manual queries
- More readable raw queries
---
### Enum type in Postgres

_Enumerated (enum) types are data types that comprise a static, ordered set
of values. They are equivalent to the enum types supported in a number of
programming languages. An example of an enum type might be the days of the
week, or a set of status values for a piece of data._
<!-- .element: class="fragment" -->

---
### Storing values as Postgres Enum
```ruby
class AddStatusToPosts < ActiveRecord::Migration[7.0]
  def up
    execute <<-SQL.squish
      CREATE TYPE posts_statuses_enum
        AS ENUM('draft', 'published', 'removed');
    SQL

    add_column :posts, :status, :posts_statuses_enum
  end
end
```
<!-- .element: class="fragment" -->
---
### Storing values as Postgres Enum
_This migration assumes that you already have a Rails enum backed by a string._

```ruby
change_column(
  :posts,
  :status,
  "posts_statuses_enum USING status::posts_statuses_enum",
)
```
<!-- .element: class="fragment" -->

Notes:
- Missing down method here, it should be pretty straight forward.
- Postgres will fail to perform the change throwing an error and you will have to remove/fix any invalid entries.

---
### Benefits of Postgres Enum
```ruby
class Post < ApplicationRecord
  enum :status, { draft: "draft", published: "published" }
end

Post.statuses
# => {"draft"=>"draft", "published"=>"published"}
```

```ruby
User
  .joins(<<~SQL.squish)
    LEFT JOINS posts
      ON posts.user_id = user.id
        AND posts.status = 'pubilshed'
  SQL
# => PG::InvalidTextRepresentation: ERROR: invalid input
#    value for enum posts_statuses_enum: "pubilshed"

Post.published.to_sql
# => SELECT "posts".* FROM "posts"
# WHERE "posts"."status" = 'published'
```
<!-- .element: class="fragment" -->

Notes:
- An enum value occupies four bytes on disk. The length of an enum value's
  textual label is limited by the NAMEDATALEN setting compiled into PostgreSQL;
  in standard builds this means at most 63 bytes.
- Important benefit: TYPE CHECK ON READING

---
### Adding new values to ENUM

```
ALTER TYPE enum_type ADD VALUE 'new_value'
ALTER TYPE enum_type ADD VALUE 'new_value' BEFORE 'old_value'
ALTER TYPE enum_type ADD VALUE 'new_value' AFTER 'old_value'
```

---
## Alternatives

### Check
<!-- .element: class="fragment" -->

```sql
CREATE TABLE posts (
  status TEXT
    CHECK (status IN ('draft', 'published', 'removed')));
```
<!-- .element: class="fragment" -->

### Relation
<!-- .element: class="fragment" -->
```sql
CREATE TABLE valid_statuses (
  id SERIAL PRIMARY KEY NOT NULL, status TEXT));

INSERT INTO valid_statuses (name) VALUES
  ('draft'), ('published'), ('removed');

CREATE TABLE posts (
  status INTEGER REFERENCES valid_statues (id));
```
<!-- .element: class="fragment" -->

Notes:
  - No ability to "type check" the syntax when reading (where) on CHECK
  - JOIN required: Values as natural keys in the reference table.
  - JOIN required: Create VIEW, VIEW is sometimes considered antipattern.

---
## ActiveRecord::PostgresEnum

```ruby
gem "activerecord-postgres_enum"
```

```ruby
create_enum :post_status, ["draft", "published", "removed"]

create_table :posts do
  t.enum :status, enum_type: :post_status
end
```
---

## Shoulda Matchers
```ruby
RSpec.describe Post, type: :model do
  let(:model) { described_class.new }

  it "#status" do
    expect(model)
      .to define_enum_for(:status)
      .backed_by_column_of_type(:enum)
      .with_values(
        draft: "draft",
        published: "published",
        removed: "removed",
      )
  end
end
```
---
## Summary

- readable queries and definitions in source code
<!-- .element: class="fragment" -->
- optimal data storage in the database
<!-- .element: class="fragment" -->
- type safety on database level on writes AND reads
<!-- .element: class="fragment" -->

---
## Questions

---
## Thank you!

