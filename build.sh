#!/bin/bash

echo "🔨 Building project..."
cargo build --release

echo "📦 Creating dist folder..."
mkdir -p dist

cp target/release/key-drop dist/

echo "✅ Build complete!"
echo "Binary located at: dist/key-drop"
