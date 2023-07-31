import os
import subprocess
import sys

base_directory = "/Users/kianenigma/Desktop/Parity/pba/assignment-3-frame-less-submissions"
temp = 0

for folder in os.listdir(base_directory):
    if folder.startswith("assignment-3-frame-less"):
        # if the a command line arg is set, make sure 'folder' contains that arg as a string, otherwise skip to next folder.
        if len(sys.argv) > 1:
            if sys.argv[1] not in folder:
                print(f"⚠️ {sys.argv[1]} not in {folder}, skipping to next folder.")
                continue
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

            # read stderr.txt, and replace any instance of "/Users/kianenigma/Desktop/Parity/pba/" in it with ""
            with open("stderr.txt", "r") as stderr_file:
              stderr_content = stderr_file.read()
              stderr_content = stderr_content.replace("/Users/kianenigma/Desktop/Parity/pba/", "")
              # If "incorrect extrinsics root in authored" is in stderr_content, increment a counter and print warning
              if "incorrect extrinsics root in authored block" in stderr_content:
                print(f"🦷 FOUND incorrect extrinsics root {full_folder_path}")
                temp += 1

              with open("stderr.txt", "w") as stderr_file:
                stderr_file.write(stderr_content)

            os.rename("stderr.txt", os.path.join(full_folder_path, "stderr.txt"))
          # if a file `result.xml` exists in the current folder, move it to `full_folder_path`.abs
          if os.path.exists("result.xml"):
            os.rename("result.xml", os.path.join(full_folder_path, "result.xml"))

            # subprocess.run(["git", "add", "."], cwd=full_folder_path)
            # subprocess.run(["git", "commit", "-m", "Add results"], cwd=full_folder_path)
            # output = subprocess.run(["git", "push"], cwd=full_folder_path)
        else:
          print(f"Could not find {wasm_file_path}, skipping to next folder.")

print(temp)
