#!/bin/bash
# rcli AI command examples
# Requires GEMINI_API_KEY environment variable set or use -k flag

echo "=== Basic AI chat ==="
rcli ai chat -p "What is Rust programming language?"

echo -e "\n=== Using different model ==="
rcli ai chat -p "Explain quantum computing" -m "gemini-1.5-pro"

echo -e "\n=== Creative writing with higher temperature ==="
rcli ai chat -p "Write a haiku about programming" --temperature 0.9

echo -e "\n=== Technical explanation with more tokens ==="
rcli ai chat -p "Explain how TCP/IP works" -t 3000

echo -e "\n=== JSON output format ==="
rcli ai chat -p "List 5 popular programming languages" -o json

echo -e "\n=== Using API key flag ==="
rcli ai chat -p "What is the capital of France?" -k "$GEMINI_API_KEY"

echo -e "\n=== Error handling example (invalid model) ==="
rcli ai chat -p "This should fail" -m "invalid-model"
