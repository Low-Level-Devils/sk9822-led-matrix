import csv
import subprocess
import sys
import os
import shutil
from pathlib import Path

def convert_gifs_to_bin(csv_file):
    try:
        with open(csv_file, 'r') as f:
            reader = csv.DictReader(f)
            
            for row in reader:
                name = row['name']
                fps = row['fps']
                
                # Build the command
                command = [
                    'cargo', 'run', '--bin', 'gif_convert',
                    '--features', 'converter', '--',
                    f'{name}.gif', name, fps
                ]
                
                print(f"Running: {' '.join(command)}")
                
                try:
                    result = subprocess.run(
                        command,
                        capture_output=True,
                        text=True,
                        check=True
                    )
                    print(f"✓ Success: {name}")
                    if result.stdout:
                        print(result.stdout)
                except subprocess.CalledProcessError as e:
                    print(f"✗ Error processing {name}: {e}")
                    if e.stderr:
                        print(e.stderr)
                
                print("-" * 50)
                
    except FileNotFoundError:
        print(f"Error: File '{csv_file}' not found")
        sys.exit(1)
    except KeyError as e:
        print(f"Error: Missing column {e} in CSV file")
        sys.exit(1)


def compile_binaries():
    try: 
        dir_path = Path("sk9822-led/examples/")

        if not dir_path.exists():
            print("Directory sk9822-led/examples/ does not exist\n")
            sys.exit(1)

        files = list(dir_path.glob(f"*.rs"))

        if not files:
            print("No example files found")
            return
        
        for file_path in files:
            example_name = file_path.stem

            command = [
                'cargo', 'build', '--release',
                '--example', example_name, 
                '--target', 'aarch64-unknown-linux-gnu'
            ]

            print(f"Compiling binary with command: {' '.join(command)}")
    
            try:
                result = subprocess.run(
                    command,
                    capture_output=True,
                    text=True,
                    check=True
                )
                print("✓ Compilation successful")
                if result.stdout:
                    print(result.stdout)
            except subprocess.CalledProcessError as e:
                print(f"✗ Compilation failed: {e}")
                if e.stderr:
                    print(e.stderr)
                sys.exit(1)

            print("-" * 50)
        
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)


def setup_output_dir():
    output_path = Path("sk9822-led-module")

    if output_path.exists():
        shutil.rmtree(output_path)

    output_path_animations = output_path / Path("animations/")
    output_path_static = output_path / Path("static/")

    output_path.mkdir(parents=True, exist_ok=True)

    print("sk9822-led-module dir created")
    print("-" * 50)


def copy_files():
    gif_binaries_input_path = Path("sk9822-led/animations")
    static_input_path = Path("sk9822-led/static")
    examples_binaries_input_path = Path("target/aarch64-unknown-linux-gnu/release/examples/")
    output_path_animations = Path("sk9822-led-module/animations")
    output_path_binaries = Path("sk9822-led-module/")
    output_path_static = Path("sk9822-led-module/static")

    if not gif_binaries_input_path.exists():
        print("Gif Binaries path sk9822-led/animations does not exist")
        sys.exit(1)

    shutil.copytree(gif_binaries_input_path, output_path_animations, dirs_exist_ok=True)

    print("Animations copied successfully")
    print("-" * 50)

    files = [f for f in examples_binaries_input_path.iterdir() if f.is_file() and f.suffix == '' and not '-' in f.name]

    for file_path in files:
        destination = output_path_binaries / file_path.name
        shutil.copy2(file_path, destination)

    print("Binaries copied successfully")
    print("-" * 50)

    shutil.copytree(static_input_path, output_path_static, dirs_exist_ok=True)

    print("Static files copied successfully")
    print("-" * 50)

if __name__ == "__main__":   
    convert_gifs_to_bin("manifest.csv")
    compile_binaries()
    setup_output_dir()
    copy_files()