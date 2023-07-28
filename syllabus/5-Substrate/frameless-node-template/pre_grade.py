import os
import subprocess

base_directory = "/Users/kianenigma/Desktop/Parity/pba/assignment-3-frame-less-submissions"

for folder in os.listdir(base_directory):
    if folder.startswith("assignment-3-frame-less"):
        full_folder_path = os.path.join(base_directory, folder)

        # fetch a branch called "wasm", if any.
        subprocess.run(["git", "fetch", "origin"], cwd=full_folder_path, stderr=subprocess.DEVNULL, stdout=subprocess.DEVNULL)

        # if wasm branch exists, switch to that branch
        checkout_output = subprocess.run(["git", "checkout", "wasm"], cwd=full_folder_path, stderr=subprocess.DEVNULL, stdout=subprocess.DEVNULL)

        if checkout_output.returncode != 0:
            print(f"⚠️ No branch called 'wasm' in {full_folder_path}, skipping to next folder.")
            continue

        # reset everything
        subprocess.run(["git", "reset", "--hard", "origin/wasm"], cwd=full_folder_path, stderr=subprocess.DEVNULL, stdout=subprocess.DEVNULL)

        # run the script
        wasm_file_path = os.path.join(full_folder_path, "runtime.wasm")
        if os.path.exists(wasm_file_path):
          stdout_file = open("./stdout.txt", "w")
          stderr_file = open("./stderr.txt", "w")
          script_path = "./pre_grade.sh"
          print(f"✅ running pre_grading on {wasm_file_path}")
          # pipe output to a file.
          subprocess.run(["bash", script_path, wasm_file_path], stdout=stdout_file, stderr=stderr_file)

          # move stdout and stderr file to full_folder_path
          if os.path.exists("stdout.txt"):
            os.rename("stdout.txt", os.path.join(full_folder_path, "stdout.txt"))
          if os.path.exists("stderr.txt"):
            # read stderr.txt and print a line in it that contains "Summary", if it exists
            with open("stderr.txt", "r") as stderr_file:
              for line in stderr_file.readlines():
                if "Summary" in line:
                  print(line)
            os.rename("stderr.txt", os.path.join(full_folder_path, "stderr.txt"))
          # if a file `result.xml` exists in the current folder, move it to `full_folder_path`.abs
          if os.path.exists("result.xml"):
            os.rename("result.xml", os.path.join(full_folder_path, "result.xml"))

            # subprocess.run(["git", "add", "result.xml"], cwd=full_folder_path)
            # subprocess.run(["git", "commit", "-m", "Add result.xml"], cwd=full_folder_path)
            # output = subprocess.run(["git", "push"], cwd=full_folder_path)
        else:
          print(f"Could not find {wasm_file_path}, skipping to next folder.")
