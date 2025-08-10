"""
Reads GOOGLE_API_KEY env key in GitHub CI and replaces them automatically with correct API KEY!

It is supposed to be called from project root! No need to call it manually, will be used from GitHub actions!

```bash
python3 scripts/secrets_for_ci.py
```
"""

import os
import yaml
import subprocess


def update_api_key(file_path):
    env_api_key = os.getenv("GOOGLE_API_KEY")

    # Read the YAML file
    with open(file_path, "r") as file:
        config = yaml.safe_load(file)

    # Update the api_key if the environment variable exists
    if env_api_key:
        config["llm"]["api_key"] = env_api_key
    else:
        print("I assume missing ENV is not intentional! If you're runing this script")
        print("Please load `GOOGLE_API_KEY` in the environment")
        print('You can do that using `export GOOGLE_API_KEY="<api key>"`')
        print("and something similar on the CI if you happen to run it on CI")
        print("No changes made")
        exit(1)

    # Write the updated config back to the file
    with open(file_path, "w") as file:
        yaml.dump(config, file, default_flow_style=False)

    try:
        subprocess.run(
            ["git", "update-index", "--assume-unchanged", "cpast_api/configuration/base.yaml"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )
    except Exception:
        pass

    print("Updated api_key successfully")


if __name__ == "__main__":
    update_api_key("cpast_api/configuration/base.yaml")
