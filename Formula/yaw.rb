class Yaw < Formula
  desc "YAml Without yaml editor"
  homepage "https://github.com/jakobwesthoff/yaw"
  url "https://github.com/jakobwesthoff/yaw/archive/refs/tags/v1.2.0.tar.gz"
  sha256 "16ec5d0086354f170a0bd8b36e8b262176213b91e2b8a6a9522f2cc655512f93" # This will be updated by the action
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
