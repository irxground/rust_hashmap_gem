require "rust_hashmap/rust_hashmap"

class Rust::HashMap
  def [](key)
    get_with_hash(key, key.hash)
  end

  def []=(key, value)
    insert_with_hash(key, key.hash, value)
  end

  def delete(key)
    remove_with_hash(key, key.hash)
  end
end
