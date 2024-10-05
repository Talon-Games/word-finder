def load_words(file_path):
    word_definitions = {}
    with open(file_path, "r") as file:
        for line in file:
            parts = line.strip().split("::", 1)
            if len(parts) > 1:
                word = parts[0].strip()
                definition = parts[1].strip()
                word_definitions[word] = definition
            else:
                word_definitions[parts[0].strip()] = ""
    return word_definitions


main_words = load_words("raw_words.txt")
clean_words = load_words("cleaned_words.txt")

merged_words = {}

for word in main_words:
    merged_words[word] = main_words[word]

for word, definition in clean_words.items():
    merged_words[word] = definition

sorted_merged_words = sorted(merged_words.items())

with open("merged.txt", "w") as output_file:
    for word, definition in sorted_merged_words:
        output_file.write(f"{word} :: {definition}\n")

print("Merged file created successfully: merged.txt")
