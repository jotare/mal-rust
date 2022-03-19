#!/usr/bin/python3

"""
Automatic Rust test generator from MAL text tests

To use, call it with a MAL test:
python3 generate-tests.py stepN_X.mal
"""

import re
import sys


if len(sys.argv) < 2:
    print("You must pass a mal test name by command line")
    exit(1)

test_file = sys.argv[1]
print(f"Using test file: '{test_file}'")

tests = []
with open(test_file) as f:
    test = {"cases": [], "comment": "DEFAULT-COMMENT"}

    for line in f:
        if re.match(r"^\s*$", line):  # blank line
            if test["cases"]:
                tests.append(test)
                test = {"cases": [], "comment": "DEFAULT-COMMENT"}
            continue

        line = line.strip()

        if line.startswith(";;;"):  # ignore comment
            continue
        elif line.startswith(";; "):
            test["comment"] = line[3:]
        elif line.startswith(";>>> "):
            continue
        elif line.startswith(";=>"):
            test_output = line[3:]
            test["cases"][-1]["output"] = test_output
        else:
            test_input = line
            test["cases"].append({
                "input": test_input
            })
    tests.append(test)

for test in tests:
    test_name = test["comment"].lower().replace(" ", "_")
    print(f"""\
#[test]
fn {test_name}() {{
    let mut env = Env::new_default();\
""")

    for case in test["cases"]:
        print(f"""\
    assert_eq!(mal_rust::rep("{case['input']}", &mut env), "{case['output']}");\
""")

    print("""\
}
""")
