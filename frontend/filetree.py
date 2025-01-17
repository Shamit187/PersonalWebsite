import os

def generate_tree(directory, indent=0, exclude=None):
    """
    Recursively generates a directory tree starting from the given directory.
    
    Args:
        directory (str): The directory path to start the tree generation.
        indent (int): The current indentation level (used for recursion).
        exclude (set): A set of directory or file names to exclude from the tree.
    """
    if exclude is None:
        exclude = {"node_modules", ".git", "__pycache__"}

    # Print the current directory
    print("  " * indent + f"|-- {os.path.basename(directory)}/")

    # List all items in the current directory
    try:
        items = os.listdir(directory)
    except PermissionError:
        print("  " * (indent + 1) + "|-- [Access Denied]")
        return

    for item in sorted(items):
        if item in exclude:
            continue

        item_path = os.path.join(directory, item)
        # Check if it's a directory
        if os.path.isdir(item_path):
            generate_tree(item_path, indent + 1, exclude)
        else:
            print("  " * (indent + 1) + f"|-- {item}")

if __name__ == "__main__":
    # Start the tree generation from the current directory
    current_directory = os.getcwd()
    print(f"Directory tree for: {current_directory}")
    generate_tree(current_directory)
sudo apt-get autoremove