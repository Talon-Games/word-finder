import json


def load_latin_words():
    with open("latin_words.json", "r") as file:
        json_file = json.load(file)
        with open("latin_words.txt", "w") as output_file:
            for word in json_file:
                defs = word["senses"]
                str_def = " ".join(defs)
                parts = word["parts"]
                if str_def.startswith("|"):
                    str_def = str_def.replace("|", "", 1)
                for part in parts:
                    if part != "---" and part != "zzz":
                        clean_part = part.replace(" | undeclined", "")
                        clean_part = clean_part.replace(" | abbreviation", "")
                        output_file.write(f"{clean_part} :: {str_def}\n")


load_latin_words()
