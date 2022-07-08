load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")


def rpi_bazel_repository(name):
    commit = "v1.0.0"
    http_archive(
        name = name,
        url = "https://github.com/mm-s/rpi_bazel/archive/refs/tags/{}.zip".format(commit),
	sha256 = "5fa9d211bfb10ee54b36c17253159af1db45da91f4f2040c3dcf45cbdf55f715",
        strip_prefix = "rpi_bazel-1.0.0",
    )
