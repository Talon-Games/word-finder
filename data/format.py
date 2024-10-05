import re

with open("./words_with_def.txt", "r") as file:
    lines = file.readlines()

with open("./cleaned_words.txt", "w") as output_file:
    line_count = 0
    for line in lines:
        line = line.strip()
        line = re.sub(r"\[.*?\]", "", line)
        line_count += 1

        parts = line.split(maxsplit=1)
        if len(parts) > 1:
            word = parts[0].lower()
            definition = parts[1]

            # remove parent words
            definition = re.sub(r"^[A-Z]+(?:,\s*[A-Z]+)*,*\s*", "", definition)
            # remove words after also
            definition = re.sub(r", also\s+[A-Z\s,]*$", "", definition)
            definition = " ".join(definition.split())
            # lowercase any other words
            definition = re.sub(
                r"\b[A-Z]{2,}\b", lambda m: m.group(0).lower(), definition
            )

            output_file.write(f"{word} :: {definition}\n")
        else:
            print(f"no def: {line_count}")
            output_file.write(f"{parts[0].lower()} :: No definition available\n")

    print(f"{line_count} words")
