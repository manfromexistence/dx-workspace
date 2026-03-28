#!/usr/bin/env python3
"""Test API keys and fetch latest models from providers."""

import os
import json
import requests
from datetime import datetime

# Provider configurations
PROVIDERS = {
    "Mistral": {
        "env_key": "MISTRAL_API_KEY",
        "url": "https://api.mistral.ai/v1/models",
        "auth_header": lambda key: {"Authorization": f"Bearer {key}"}
    },
    "Groq": {
        "env_key": "GROQ_API_KEY",
        "url": "https://api.groq.com/openai/v1/models",
        "auth_header": lambda key: {"Authorization": f"Bearer {key}"}
    },
    "Cerebras": {
        "env_key": "CEREBRAS_API_KEY",
        "url": "https://api.cerebras.ai/v1/models",
        "auth_header": lambda key: {"Authorization": f"Bearer {key}"}
    },
    "OpenRouter": {
        "env_key": "OPENROUTER_API_KEY",
        "url": "https://openrouter.ai/api/v1/models",
        "auth_header": lambda key: {"Authorization": f"Bearer {key}"}
    },
    "DeepSeek": {
        "env_key": "DEEPSEEK_API_KEY",
        "url": "https://api.deepseek.com/v1/models",
        "auth_header": lambda key: {"Authorization": f"Bearer {key}"}
    },
    "Together": {
        "env_key": "TOGETHER_API_KEY",
        "url": "https://api.together.xyz/v1/models",
        "auth_header": lambda key: {"Authorization": f"Bearer {key}"}
    },
    "Gemini": {
        "env_key": "GOOGLE_GENERATIVE_AI_API_KEY",
        "url": "https://generativelanguage.googleapis.com/v1beta/models",
        "auth_header": lambda key: {},
        "params": lambda key: {"key": key}
    },
    "Cohere": {
        "env_key": "COHERE_API_KEY",
        "url": "https://api.cohere.com/v1/models",
        "auth_header": lambda key: {"Authorization": f"Bearer {key}"}
    },
    "SambaNova": {
        "env_key": "SAMBANOVA_API_KEY",
        "url": "https://api.sambanova.ai/v1/models",
        "auth_header": lambda key: {"Authorization": f"Bearer {key}"}
    },
    "HuggingFace": {
        "env_key": "HUGGINGFACE_API_KEY",
        "url": "https://api-inference.huggingface.co/models",
        "auth_header": lambda key: {"Authorization": f"Bearer {key}"}
    },
}

def load_env():
    """Load .env file."""
    env = {}
    try:
        with open(".env", 'r', encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                if line and not line.startswith('#') and '=' in line:
                    key, value = line.split('=', 1)
                    env[key] = value.strip('"').strip("'")
    except Exception as e:
        print(f"Error loading .env: {e}")
    return env

def test_provider(name, config, api_key):
    """Test a provider's API key and fetch models."""
    try:
        headers = config["auth_header"](api_key)
        params = config.get("params", lambda k: {})(api_key)
        response = requests.get(config["url"], headers=headers, params=params, timeout=15)
        
        if response.status_code == 200:
            data = response.json()
            models = []
            
            if "data" in data:
                models = [m.get("id") for m in data["data"] if "id" in m]
            elif "models" in data:
                models = [m.get("name") for m in data["models"] if "name" in m]
            elif isinstance(data, list):
                models = [m.get("id") or m.get("name") for m in data if isinstance(m, dict)]
            
            return True, models, None
        else:
            return False, [], f"HTTP {response.status_code}"
    except Exception as e:
        return False, [], str(e)

def main():
    print("="*70)
    print("API Key Testing Tool")
    print("="*70)
    print()
    
    env = load_env()
    results = {}
    
    for name, config in PROVIDERS.items():
        print(f"Testing {name}...")
        
        api_key = env.get(config["env_key"])
        if not api_key:
            print(f"  [SKIP] No API key found")
            results[name] = {"status": "no_key"}
            continue
        
        success, models, error = test_provider(name, config, api_key)
        
        if success:
            print(f"  [OK] Found {len(models)} models")
            if models:
                for model in models[:3]:
                    print(f"    - {model}")
                if len(models) > 3:
                    print(f"    ... and {len(models)-3} more")
            results[name] = {"status": "working", "models": models}
        else:
            print(f"  [FAIL] {error}")
            results[name] = {"status": "failed", "error": error}
        print()
    
    # Save results
    with open("api_test_results.json", 'w') as f:
        json.dump({"timestamp": datetime.now().isoformat(), "results": results}, f, indent=2)
    
    print("="*70)
    print("Results saved to api_test_results.json")
    print("="*70)

if __name__ == "__main__":
    main()
