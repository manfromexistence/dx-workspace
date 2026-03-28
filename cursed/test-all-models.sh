#!/usr/bin/env bash
# Test all Qwen models and compare performance

echo "🧪 Testing All Qwen Models"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

TEST_PROMPT="Write a Python function to calculate the factorial of a number. Keep it simple and efficient."

MODELS=(
    "qwen2.5:0.5b"
    "qwen3:0.6b"
    "qwen3.5:0.8b"
    "qwen3.5:2b"
)

for model in "${MODELS[@]}"; do
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Testing: $model"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    # Measure time
    START=$(date +%s)
    
    # Run test
    echo "$TEST_PROMPT" | ollama run "$model" 2>&1
    
    END=$(date +%s)
    DURATION=$((END - START))
    
    echo ""
    echo "⏱️  Time taken: ${DURATION} seconds"
    echo ""
    
    # Small delay between tests
    sleep 2
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ All tests complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
