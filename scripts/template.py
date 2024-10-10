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

mods, top, pairs, bottom = re.fullmatch(
    r"((?:mod \w+;\n)+)(.*?)(?:( {8}.* -> String\),\n)+)(.*)",
    open(MOD_FILENAME).read(),
    re.DOTALL | re.MULTILINE,
).groups()


new_mod = f"mod {name};"
assert new_mod not in mods
mods = "\n".join(sorted(mods.splitlines() + [new_mod]))

new_pair = f'        ("{name}".to_string(), {name}::subject as fn() -> String),'
pairs = "\n".join(
    sorted(
        pairs.splitlines() + [new_pair],
        key=lambda s: s.split('"')[1],
    )
)

open(MOD_FILENAME, "w").write(mods + "\n" + top + pairs + "\n" + bottom)
open(problem_filename, "w").write(TEMPLATE)
os.system(f"code {problem_filename}")
