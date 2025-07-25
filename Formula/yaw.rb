class Yaw < Formula
  desc "YAml Without yaml editor"
  homepage "https://github.com/jakobwesthoff/yaw"
  url "https://github.com/jakobwesthoff/yaw/archive/refs/tags/v1.0.2.tar.gz"
  sha256 "2ca5ee5e67285ce5442183880456680ff6e2fe7a1d9e6d03a96f36be6dcfa16c" # This will be updated by the action
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
