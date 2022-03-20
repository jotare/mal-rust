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
    tests_content = f.readlines()

test = {"cases": [], "comment": "DEFAULT-COMMENT"}
lineno = 0
while lineno + 1 < len(tests_content):
    lineno += 1
    line = tests_content[lineno]

    if re.match(r"^\s*$", line):  # blank line
        if test["cases"]:
            tests.append(test)
            test = {"cases": [], "comment": "DEFAULT-COMMENT"}
        continue

    line = line.strip()

    if line.startswith(";;;"):  # ignore comment
        continue
    elif line.startswith(";; "):
        test["comment"] = line[3:].replace("-", "_") \
                                  .replace("*", "")  \
                                  .replace("!", "")
    elif line.startswith(";;"):
        continue
    elif line.startswith(";>>> "):
        continue
    else:
        test_input = ""
        test_output = ""
        while lineno < len(tests_content):
            line = tests_content[lineno].strip()
            if line.startswith(";=>"):
                test_output += line[3:]
                break
            elif line.startswith(";/"):
                test_output = line[2:]
                print("ERR: ", test_output)
                break
            else:
                test_input += line + "\n"

            lineno += 1

        test["cases"].append({
            "input": test_input.strip(),
            "output": test_output.strip()
        })


for test in tests:
    test_name = test["comment"].lower().replace(" ", "_")
    print(f"""\
#[test]
fn {test_name}() {{
    let mut env = Env::new_default();\
""")

    for case in test["cases"]:
        input = repr(case["input"]).strip("'")
        output = repr(case["output"]).strip("'")

        print(f"""\
    assert_eq!(
        mal_rust::rep("{input}", &mut env),
        "{output}"
    );\
""")

    print("""\
}
""")
