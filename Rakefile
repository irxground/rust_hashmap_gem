require "rspec/core/rake_task"
RSpec::Core::RakeTask.new(:spec)

require_relative "ext/build_task.rb"
BuildTask.new("rust_hashmap", __dir__, File.join(__dir__, "lib"))

task :fmt do
  system "cargo fmt"
  system "rufo ."
end

task :default => [:clobber, :install_debug, :spec]
