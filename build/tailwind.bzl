"""Bazel rules for building Tailwind CSS."""

def _tailwind_css_impl(ctx):
    output = ctx.actions.declare_file(ctx.attr.out)
    
    # Collect all source files for content scanning
    srcs = []
    for src in ctx.attr.srcs:
        srcs.extend(src.files.to_list())
    
    # Create a file with the list of source files for content scanning
    content_files = ctx.actions.declare_file(ctx.attr.name + "_content_files.txt")
    ctx.actions.write(
        output = content_files,
        content = "\n".join([f.path for f in srcs]),
    )
    
    # Build the command
    args = ctx.actions.args()
    args.add("-i", ctx.file.input.path)
    args.add("-o", output.path)
    if ctx.attr.minify:
        args.add("--minify")
    
    # Add content paths (scan the source files for Tailwind classes)
    for src in srcs:
        args.add("--content", src.path)
    
    ctx.actions.run(
        inputs = [ctx.file.input] + srcs,
        outputs = [output],
        executable = ctx.executable._tailwindcss,
        arguments = [args],
        mnemonic = "TailwindCSS",
        progress_message = "Building Tailwind CSS %s" % output.short_path,
    )
    
    return [DefaultInfo(files = depset([output]))]

tailwind_css = rule(
    implementation = _tailwind_css_impl,
    attrs = {
        "input": attr.label(
            allow_single_file = [".css"],
            mandatory = True,
            doc = "The input CSS file with Tailwind directives",
        ),
        "out": attr.string(
            mandatory = True,
            doc = "The output CSS filename",
        ),
        "srcs": attr.label_list(
            allow_files = True,
            doc = "Source files to scan for Tailwind classes",
        ),
        "minify": attr.bool(
            default = True,
            doc = "Whether to minify the output",
        ),
        "_tailwindcss": attr.label(
            default = "@tailwindcss_macos_arm64//file",
            executable = True,
            cfg = "exec",
            doc = "The Tailwind CSS CLI executable",
        ),
    },
)
