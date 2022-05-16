#!/usr/bin/env ruby

require "dry-types"

PostStatuses = Dry::Types["string"].enum("draft", "published", "removed")
PostStatuses["draft"] # => "draft"
PostStatuses["other"] # => "draft"
PostStatuses.values # => ["draft", "published", "removed"]