"""
This is a tool that generates a .worlds file from a text format
# is a tile
p is the player
' ' is an empty spot
"""
import sys
import os

#level1 = """
#""".replace("\n", "")

level_file_names = os.listdir("./tools/levels")
levels_content = []
for level_file_name in level_file_names:
    with open(os.path.join("./tools/levels", level_file_name), "r") as f:
        levels_content.append(f.read().replace("\n", ""))

levels = "|".join(levels_content)

# write to the .level file
with open(sys.argv[1], "w") as f:
    f.write(levels)
