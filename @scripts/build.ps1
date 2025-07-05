Push-Location ../
Remove-Item -Path public/pkg -Recurse -Force -ErrorAction SilentlyContinue
wasm-pack build --target web --out-dir public/pkg
Pop-Location