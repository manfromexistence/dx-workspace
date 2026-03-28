# Setup Mistral as default provider for Codex
$ErrorActionPreference = "Stop"

Write-Host "Setting up Mistral as default provider..." -ForegroundColor Cyan

# Load .env file
$envContent = Get-Content .env -Raw
if ($envContent -match 'MISTRAL_API_KEY="([^"]+)"') {
    $mistralKey = $matches[1]
} else {
    Write-Host "MISTRAL_API_KEY not found in .env" -ForegroundColor Red
    exit 1
}

Write-Host "Found Mistral API key" -ForegroundColor Green

# Fetch models from Mistral
Write-Host "Fetching models from Mistral API..." -ForegroundColor Cyan

$headers = @{
    "Authorization" = "Bearer $mistralKey"
    "Content-Type" = "application/json"
}

try {
    $response = Invoke-RestMethod -Uri "https://api.mistral.ai/v1/models" -Headers $headers -Method Get
    $models = $response.data | Sort-Object -Property created -Descending
    
    if ($models.Count -eq 0) {
        Write-Host "No models returned, using default" -ForegroundColor Yellow
        $defaultModel = "mistral-small-latest"
    } else {
        $defaultModel = $models[0].id
        Write-Host "Found $($models.Count) models" -ForegroundColor Green
        Write-Host "Selected model: $defaultModel" -ForegroundColor Cyan
    }
} catch {
    Write-Host "Could not fetch models: $_" -ForegroundColor Yellow
    Write-Host "Using default model instead..." -ForegroundColor Yellow
    $defaultModel = "mistral-small-latest"
}

# Update config.toml
$configPath = "$env:USERPROFILE\.codex\config.toml"

if (-not (Test-Path $configPath)) {
    Write-Host "Creating new config file..." -ForegroundColor Cyan
    New-Item -Path (Split-Path $configPath) -ItemType Directory -Force | Out-Null
}

# Read existing config or create new one
if (Test-Path $configPath) {
    $config = Get-Content $configPath -Raw
} else {
    $config = ""
}

# Update or add model_provider and model
if ($config -match 'model_provider\s*=') {
    $config = $config -replace 'model_provider\s*=\s*"[^"]*"', 'model_provider = "mistral"'
} else {
    $config = 'model_provider = "mistral"' + "`n" + $config
}

if ($config -match 'model\s*=') {
    $config = $config -replace 'model\s*=\s*"[^"]*"', "model = `"$defaultModel`""
} else {
    # Add after model_provider line
    $config = $config -replace '(model_provider\s*=\s*"[^"]*")', "`$1`nmodel = `"$defaultModel`""
}

# Write config
Set-Content -Path $configPath -Value $config -NoNewline

Write-Host ""
Write-Host "Configuration updated successfully!" -ForegroundColor Green
Write-Host "Config location: $configPath" -ForegroundColor Cyan
Write-Host ""
Write-Host "Current configuration:" -ForegroundColor Cyan
Write-Host "  Provider: mistral" -ForegroundColor White
Write-Host "  Model: $defaultModel" -ForegroundColor White
Write-Host ""
Write-Host "You can now run: just dx" -ForegroundColor Green
