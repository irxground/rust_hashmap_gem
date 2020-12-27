require "json"
require "rake/tasklib"

class BuildTask < Rake::TaskLib
  attr_reader :name, :lib_dir
  attr_reader :manifest_file

  def initialize(name, cargo_dir, lib_dir)
    @name = name
    @lib_dir = lib_dir
    @manifest_file = File.join(cargo_dir, "Cargo.toml")

    define_tasks
  end

  def define_tasks
    target_so_dir = File.join(lib_dir, name)
    target_so = File.join(target_so_dir, "#{name}.#{RbConfig::CONFIG["DLEXT"]}")

    task :cargo do
      unless system "cargo", "-V"
        raise "`cargo` command not found. Please install Rust compiler: https://www.rust-lang.org/"
      end
    end

    desc "Remove build cache"
    task :clean => :cargo do
      sh "cargo", "clean", "--manifest-path", manifest_file
    end

    desc "Remove build cache and compiled library"
    task :clobber => :clean do
      rm_f target_so
    end

    desc "Compile native extension (optimized)"
    task :build => :cargo do
      build("release")
    end

    desc "Compile native extension (NOT optimized)"
    task :build_debug => :cargo do
      build("debug")
    end

    directory target_so_dir

    desc "Place native library (optimized)"
    task :install => [:build, target_so_dir] do
      cp cargo_output("release"), target_so, preserve: true, verbose: true
    end

    desc "Place native library (NOT optimized)"
    task :install_debug => [:build_debug, target_so_dir] do
      cp cargo_output("debug"), target_so, preserve: true, verbose: true
    end
  end

  def build(profile)
    env = { "RUSTFLAGS" => "-C target-cpu=native" }
    if RUBY_PLATFORM =~ /mingw/
      env["RUSTUP_TOOLCHAIN"] = "stable-#{RbConfig::CONFIG["host_cpu"]}-pc-windows-gnu"
    end
    cmds = ["cargo", "build", "--manifest-path", manifest_file]
    cmds.concat ["--release"] if profile == "release"
    cmds.concat ["--features", "method_cache"] if ENV["METHOD_CACHE"]
    sh env, *cmds
  end

  def cargo_output(profile)
    metadata = JSON.parse(`cargo metadata --format-version=1 --manifest-path #{manifest_file}`)
    cargo_name = metadata.dig("packages", 0, "name")
    filename = case RUBY_PLATFORM
      when /mingw/; "#{cargo_name}.dll"
      when /darwin/; "lib#{cargo_name}.dylib"
      when /linux/; "lib#{cargo_name}.so"
      end
    File.join(metadata["target_directory"], profile, filename)
  end
end
