#!/usr/bin/env python3
"""
Test API keys and update models.json with latest models from each provider.
This script tests all API keys from .env and fetches the latest available models.
"""

import os
import json
import requests
from typing import Dict, List, Optional
from dataclasses import dataclass
from datetime import datetime

@dataclass
class ProviderConfig:
    name: str
    env_key: str
    api_base: str
    models_endpoint: str
    headers_func: callable
    test_model: Optional[str] = None

# Provider configurations
PROVIDERS = [
    ProviderConfig(
        name="Mistral",
        env_key="MISTRAL_API_KEY",
        api_base="https://api.mistral.ai/v1",
        models_endpoint="/models",
        headers_func=lambda key: {"Authorization": f"Bearer {key}"},
        test_model="mistral-small-latest"
    ),
    ProviderConfig(
        name="Google Gemini",
        env_key="GOOGLE_GENERATIVE_AI_API_KEY",
        api_base="https://generativelanguage.googleapis.com/v1beta",
        models_endpoint="/models",
        headers_func=lambda key: {},  # Key goes in URL
        test_model="gemini-pro"
    ),
    ProviderConfig(
        name="Groq",
        env_key="GROQ_API_KEY",
        api_base="https://api.groq.com/openai/v1",
        models_endpoint="/models",
        headers_func=lambda key: {"Authorization": f"Bearer {key}"},
        test_model="llama-3.3-70b-versatile"
    ),
    ProviderConfig(
        name="Cerebras",
        env_key="CEREBRAS_API_KEY",
        api_base="https://api.cerebras.ai/v1",
        models_endpoint="/models",
        headers_func=lambda key: {"Authorization": f"Bearer {key}"},
        test_model="llama3.1-8b"
    ),
    ProviderConfig(
        name="OpenRouter",
        env_key="OPENROUTER_API_KEY",
        api_base="https://openrouter.ai/api/v1",
        models_endpoint="/models",
        headers_func=lambda key: {"Authorization": f"Bearer {key}"},
        test_model="meta-llama/llama-3.2-3b-instruct:free"
    ),
    ProviderConfig(
        name="Cohere",
        env_key="COHERE_API_KEY",
        api_base="https://api.cohere.ai/v1",
        models_endpoint="/models",
        headers_func=lambda key: {"Authorization": f"Bearer {key}"},
        test_model="command"
    ),
    ProviderConfig(
        name="DeepSeek",
        env_key="DEEPSEEK_API_KEY",
        api_base="https://api.deepseek.com/v1",
        models_endpoint="/models",
        headers_func=lambda key: {"Authorization": f"Bearer {key}"},
        test_model="deepseek-chat"
    ),
    ProviderConfig(
        name="Together AI",
        env_key="TOGETHER_API_KEY",
        api_base="https://api.together.xyz/v1",
        models_endpoint="/models",
        headers_func=lambda key: {"Authorization": f"Bearer {key}"},
        test_model="meta-llama/Llama-3-8b-chat-hf"
    ),
]

def load_env_file(filepath: str = ".env") -> Dict[str, str]:
    """Load environment variables from .env file."""
    env_vars = {}
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                if line and not line.startswith('#') and '=' in line:
                    key, value = line.split('=', 1)
                    # Remove quotes if present
                    value = value.strip('"').strip("'")
                    env_vars[key] = value
    except FileNotFoundError:
        print(f"❌ .env file not found at {filepath}")
    return env_vars

def test_api_key(provider: ProviderConfig, api_key: str) -> tuple[bool, Optional[List[str]], Optional[str]]:
    """
    Test if an API key works and fetch available models.
    Returns: (success, models_list, error_message)
    """
    try:
        headers = provider.headers_func(api_key)
        
        # Special handling for Google Gemini
        if provider.name == "Google Gemini":
            url = f"{provider.api_base}{provider.models_endpoint}?key={api_key}"
        else:
            url = f"{provider.api_base}{provider.models_endpoint}"
        
        print(f"  Testing {provider.name}...")
        response = requests.get(url, headers=headers, timeout=10)
        
        if response.status_code == 200:
            data = response.json()
            
            # Extract model IDs based on provider response format
            models = []
            if "data" in data:  # OpenAI-compatible format
                models = [m.get("id") for m in data["data"] if "id" in m]
            elif "models" in data:  # Google Gemini format
                models = [m.get("name", "").replace("models/", "") for m in data["models"]]
            elif isinstance(data, list):  # Direct list format
                models = [m.get("id") or m.get("name") for m in data if isinstance(m, dict)]
            
            return True, models, None
        else:
            return False, None, f"HTTP {response.status_code}: {response.text[:200]}"
            
    except requests.exceptions.Timeout:
        return False, None, "Request timeout"
    except requests.exceptions.RequestException as e:
        return False, None, str(e)
    except Exception as e:
        return False, None, f"Unexpected error: {str(e)}"

def main():
    print("=" * 70)
    print("API Key Testing and Model Discovery Tool")
    print("=" * 70)
    print()
    
    # Load environment variables
    print("📂 Loading .env file...")
    env_vars = load_env_file()
    
    if not env_vars:
        print("❌ No environment variables loaded. Exiting.")
        return
    
    print(f"✓ Loaded {len(env_vars)} environment variables")
    print()
    
    # Test each provider
    results = {}
    working_providers = []
    
    for provider in PROVIDERS:
        print(f"🔍 Testing {provider.name}")
        print("-" * 70)
        
        api_key = env_vars.get(provider.env_key)
        
        if not api_key:
            print(f"  ⚠️  API key not found in .env: {provider.env_key}")
            results[provider.name] = {"status": "missing_key", "models": []}
            print()
            continue
        
        # Mask the API key for display
        masked_key = api_key[:10] + "..." if len(api_key) > 10 else "***"
        print(f"  Key: {masked_key}")
        
        success, models, error = test_api_key(provider, api_key)
        
        if success:
            print(f"  ✅ API key works!")
            print(f"  📋 Found {len(models) if models else 0} models")
            if models:
                print(f"  Latest models:")
                for model in models[:5]:  # Show first 5
                    print(f"    - {model}")
                if len(models) > 5:
                    print(f"    ... and {len(models) - 5} more")
            
            results[provider.name] = {
                "status": "working",
                "models": models or [],
                "api_key": masked_key
            }
            working_providers.append(provider.name)
        else:
            print(f"  ❌ API key failed: {error}")
            results[provider.name] = {
                "status": "failed",
                "error": error,
                "models": []
            }
        
        print()
    
    # Summary
    print("=" * 70)
    print("SUMMARY")
    print("=" * 70)
    print(f"✅ Working providers: {len(working_providers)}/{len(PROVIDERS)}")
    for name in working_providers:
        print(f"  - {name}")
    
    failed = [p.name for p in PROVIDERS if results.get(p.name, {}).get("status") == "failed"]
    if failed:
        print(f"\n❌ Failed providers: {len(failed)}")
        for name in failed:
            print(f"  - {name}")
    
    # Save results
    output_file = "api_test_results.json"
    with open(output_file, 'w') as f:
        json.dump({
            "timestamp": datetime.now().isoformat(),
            "results": results
        }, f, indent=2)
    
    print(f"\n📄 Detailed results saved to: {output_file}")
    print()
    print("=" * 70)
    print("NEXT STEPS")
    print("=" * 70)
    print("1. Review the results above")
    print("2. Check api_test_results.json for full model lists")
    print("3. Update codex-rs/core/models.json with the latest models")
    print("4. Test the updated models in your TUI")
    print()

if __name__ == "__main__":
    main()
