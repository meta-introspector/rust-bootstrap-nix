import sys
import os
import json
import time
import argparse

# Add the directory containing bootstrap.py to the Python path
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "src", "bootstrap"))

import bootstrap

def main():
    parser = argparse.ArgumentParser(description="Generate JSON output for Nix build.")
    parser.add_argument("--output-dir", required=True, help="Directory to write the JSON output file.")
    args = parser.parse_args()

    generated_filenames = []

    for i in range(3): # Generate 3 dummy JSON files
        # Dummy arguments for the run function
        dummy_args = [f"rustc_{i}", "--version"]
        dummy_kwargs = {"env": {"TEST_VAR": f"test_value_{i}"}, "cwd": f"/tmp/{i}"}

        # Generate a unique filename for the JSON output
        output_filename = f"xpy_json_output_{int(time.time())}_{i}.json"

        # Call the run function directly, passing the output directory and filename
        bootstrap.run(dummy_args, output_dir=args.output_dir, output_filename=output_filename, dry_run_nix_json=True, **dummy_kwargs)

        generated_filenames.append(output_filename)

    # Print the names of the generated files to stdout so the shell script can capture them
    print(" ".join(generated_filenames))

if __name__ == '__main__':
    main()
