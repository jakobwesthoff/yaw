class Yaw < Formula
  desc "YAml Without yaml editor"
  homepage "https://github.com/jakobwesthoff/yaw"
  url "https://github.com/jakobwesthoff/yaw/archive/refs/tags/v1.1.0.tar.gz"
  sha256 "07d624e98b5dd1c9b42cc34bd2d60ee4094781bcd71277b7d97307ba10ae4f1b" # This will be updated by the action
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Test that the binary exists and is executable
    assert_predicate bin/"yaw", :exist?
    assert_predicate bin/"yaw", :executable?
  end
end
