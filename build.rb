#!/usr/bin/ruby

#
# The AssemblyLift build script
#

require 'shellwords'

# Check environment
# Need: docker, cargo, rustc
#

def check_exists(executable)
  `#{executable} &> /dev/null`

  if $?.exitstatus != 0
    puts "Could not exec #{executable}"
    return false
  else
    puts "Found #{executable}!"
    return true
  end
end

def die(message)
  puts "DIE: " + message
  exit(-1)
end

has_docker = check_exists("docker")
has_cargo = check_exists("cargo")
has_rustc = check_exists("rustc")

if not has_docker or not has_cargo or not has_rustc
  die("Missing build dependency, exiting...")
end

DOCKER = "docker"
CARGO = "cargo"
RUSTC = "rustc"

# Check first argument

args = %w[build test]
arg_error_string = "build.rb must be run with one of #{args} as an argument"

unless ARGV[0]
  die(arg_error_string)
end

cmd = ARGV[0]

unless args.include?(cmd)
  die(arg_error_string)
end

# Switch on commands
case cmd
when "build"
  build_args = %w[local deploy]
  build_arg_error_string = "build.rb build command must be run with one of #{build_args} as an argument"

  build_cmd = ARGV[1]

  unless build_args.include?(build_cmd)
    die(build_arg_error_string)
  end

  case build_cmd
  when "local"
    super_args = ARGV[2..ARGV.length].map{|arg| Shellwords.escape arg}.join(' ')
    `#{CARGO} build #{super_args}`

  when "deploy"
    die("deploy-mode build is not yet implemented")

  else
    die(build_arg_error_string)
  end

when "test"
  die("test is not yet implemented")

else
  die(arg_error_string)
end
