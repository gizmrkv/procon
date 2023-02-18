import glob
import os

paths = [path for path in glob.glob("src/**", recursive=True) if os.path.isfile(path)]
out_path = "integrated.rs"

use_libs = ["std::io::*", "std::collections::*", "std::cmp::*", "std::ops::*"]
blacklist = ["use ", "mod ", "///"]
test_flag = "#[cfg(test)]\n"

with open(out_path, mode="w", encoding="utf-8") as out_file:
    out_file.writelines([f"use {lib};\n" for lib in use_libs])
    for path in paths:
        with open(path, mode="r", encoding="utf-8") as file:
            lines = file.readlines()
            if test_flag in lines:
                end = lines.index(test_flag)
                lines = lines[:end]

            out_file.writelines(
                [
                    line
                    for line in lines
                    if all([word not in line for word in blacklist])
                ]
            )
