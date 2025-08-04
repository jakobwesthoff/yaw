class Yaw < Formula
  desc "YAml Without yaml editor"
  homepage "https://github.com/jakobwesthoff/yaw"
  url "https://github.com/jakobwesthoff/yaw/archive/refs/tags/v1.3.0.tar.gz"
  sha256 "80160953dbf20aeb5137eb2c40cb04b23c541f28711ce88b9b0e35f02bea7222" # This will be updated by the action
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
