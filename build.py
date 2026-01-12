import csv
import subprocess
import sys

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


def compile_binary(example_name):
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

if __name__ == "__main__":   
    convert_gifs_to_bin("manifest.csv")
    compile_binary("animation_test")