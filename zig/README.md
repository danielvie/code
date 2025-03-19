# how add git dependency on zig build project

EXAMPLE: (raylib-zig)

```sh
zig fetch --save git+https://github.com/Not-Nik/raylib-zig#devel
```

Then add raylib-zig as a dependency and import its modules and artifact in your build.zig:
```zig
const raylib_dep = b.dependency("raylib_zig", .{
    .target = target,
    .optimize = optimize,
});

const raylib = raylib_dep.module("raylib"); // main raylib module
const raygui = raylib_dep.module("raygui"); // raygui module
const raylib_artifact = raylib_dep.artifact("raylib"); // raylib C library
```

And add the modules and artifact to target
```zig
exe.linkLibrary(raylib_artifact);
exe.root_module.addImport("raylib", raylib);
exe.root_module.addImport("raygui", raygui);
```
