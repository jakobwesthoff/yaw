class Yaw < Formula
  desc "YAml Without yaml editor"
  homepage "https://github.com/jakobwesthoff/yaw"
  url "https://github.com/jakobwesthoff/yaw/archive/refs/tags/v1.0.1.tar.gz"
  sha256 "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef" # This will be updated by the action
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
