import glob
import os
import argparse

parser = argparse.ArgumentParser()
parser.add_argument("--main_path", type=str)

args = parser.parse_args()

paths = [path for path in glob.glob("src/**", recursive=True) if os.path.isfile(path)]
paths.append(args.main_path)
out_path = "integrated.rs"

use_libs = ["std::io::*", "std::collections::*", "std::cmp::*", "std::ops::*"]
blacklist = ["use ", "mod ", "///"]
test_flag = "#[cfg(test)]\n"

with open(out_path, mode="w", encoding="utf-8") as out_file:
    out_file.writelines([f"use {lib};\n" for lib in use_libs])
    for path in paths:
        if "/bin/" in path:
            continue

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
