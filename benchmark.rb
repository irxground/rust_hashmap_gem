# https://gist.github.com/wrzasa/6b456f73012ce98ae6feb6aaa4ba933e

require 'benchmark'
require 'optparse'
require "bundler/setup"
require "rust_hashmap"

use_rust = false
seed = nil
opt = OptionParser.new
opt.on('-r', '--[no-]rust') {|v| use_rust = true }
opt.on('-s', '--seed VALUE') {|v| seed = v.to_i }
opt.parse!(ARGV)

puts "Hash: #{use_rust ? "Rust" : "Original"}"
if seed
  puts "Seed: #{seed}"
end

h = use_rust ? Rust::HashMap.new : {}
1000.times do |i|
  h[i] = i.to_s
end

count = 50_000_000
# count = 5_000_000

indexes = count.times.map { rand(1000) }

h2 = use_rust ? Rust::HashMap.new : {}
Benchmark.bm do |x|
  x.report("values") { 500_000.times { h.values } }
  x.report("keys  ") { 500_000.times { h.keys } }
  x.report("find  ") { indexes.each { |i| h[i] } }
  x.report("insert") { indexes.each { |i| h2[i] = i.to_s } }
  x.report("delete") { indexes.each { |i| h.delete(i) } }
end
