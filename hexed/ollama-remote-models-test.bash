curl http://localhost:11434/api/chat -d '{
  "model": "gpt-oss:120b-cloud",
  "messages": [{ "role": "user", "content": "what is 2+2 in math!" }],
  "stream": false
}'

PS F:\codex> $response = Invoke-RestMethod -Uri "https://api.cerebras.ai/v1/chat/completions" -Method Post -Headers @{"Authorization"="Bearer csk-v4prvcwf46pv3jvwr355c8twp6yf8pwrrftn8eexm6d3kn5e"; "Content-Type"="application/json"} -Body $body
PS F:\codex> $response.choices[0].message.content
Hello. Is there something I can help you with or would you like to chat?