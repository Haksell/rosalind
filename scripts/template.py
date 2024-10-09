import os
import re
import sys

TEMPLATE = """\
pub fn subject() -> String {
    solve().to_string()
}

fn solve() -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 42);
    }
}
"""

MOD_FILENAME = "src/problems/mod.rs"

assert len(sys.argv) == 2
name = sys.argv[1]

problem_filename = f"src/problems/{name}.rs"

mods, top, inserts, bottom = re.fullmatch(
    r"((?:mod \w+;\n)+)(.*?)((?: +map\.insert[^\n]*;\n)+)(.*)",
    open(MOD_FILENAME).read(),
    re.DOTALL | re.MULTILINE,
).groups()

new_mod = f"mod {name};"
assert new_mod not in mods
mods = "\n".join(sorted(mods.splitlines() + [new_mod]))

new_insert = (
    f'        map.insert("{name}".to_string(), {name}::subject as fn() -> String);'
)
inserts = "\n".join(
    sorted(
        inserts.splitlines() + [new_insert],
        key=lambda s: s.split('"')[1],
    )
)

open(MOD_FILENAME, "w").write(mods + "\n" + top + inserts + "\n" + bottom)
open(problem_filename, "w").write(TEMPLATE)
os.system(f"code {problem_filename}")
