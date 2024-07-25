import json
import zlib


def en():
    with open("en.z", "rb") as f:
        f.seek(3606842)
        bytes = f.read(3609317 - 3606842)

        s = zlib.decompress(bytes).decode("utf-8")

        list_obj = s.split("|")

        word = {}
        word["word"] = list_obj[0]
        word["id"] = list_obj[1]
        word["pronunciation"] = {}
        if list_obj[2]:
            word["pronunciation"]["美"] = list_obj[2]
        if list_obj[3]:
            word["pronunciation"]["英"] = list_obj[3]
        if list_obj[4]:
            word["pronunciation"][""] = list_obj[4]
        word["paraphrase"] = json.loads(list_obj[5])
        word["rank"] = list_obj[6]
        word["pattern"] = list_obj[7]
        word["sentence"] = json.loads(list_obj[8])

        print(json.dumps(word))


def zh():
    with open("zh.z", "rb") as f:
        f.seek(41670)
        bytes_obj = f.read(42231 - 41670)

        str_obj = zlib.decompress(bytes_obj).decode("utf8")
        list_obj = str_obj.split("|")
        word = {}
        word["word"] = list_obj[0]
        word["id"] = list_obj[1]
        word["pronunciation"] = ""
        if list_obj[2]:
            word["pronunciation"] = list_obj[2]
        word["paraphrase"] = json.loads(list_obj[3])
        word["desc"] = []
        if list_obj[4]:
            word["desc"] = json.loads(list_obj[4])
        word["sentence"] = []
        if list_obj[5]:
            word["sentence"] = json.loads(list_obj[5])
        print(json.dumps(word))


zh()
