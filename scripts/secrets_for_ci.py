"""
Reads GOOGLE_API_KEY env key in GitHub CI and replaces them automatically with correct API KEY!

It is supposed to be called from project root! No need to call it manually, will be used from GitHub actions!

```bash
python3 scripts/secrets_for_ci.py
```
"""
import os
import yaml

def update_api_key(file_path):
    env_api_key = os.getenv("GOOGLE_API_KEY")
    
    # Read the YAML file
    with open(file_path, "r") as file:
        config = yaml.safe_load(file)
    
    # Update the api_key if the environment variable exists
    if env_api_key:
        config["llm"]["api_key"] = env_api_key
    
    # Write the updated config back to the file
    with open(file_path, "w") as file:
        yaml.dump(config, file, default_flow_style=False)
    
    print("Updated api_key successfully" if env_api_key else "No changes made")

if __name__ == "__main__":
    update_api_key("cpast_api/configuration/base.yaml")
