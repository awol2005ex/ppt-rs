#!/bin/bash
# Example script: Generate PPTX files from Markdown examples

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║     Generating PPTX from Markdown Examples                ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo

# Create output directory
mkdir -p examples/output

# Generate presentations from markdown files
echo "1. Generating Rust presentation..."
cargo run -- from-md examples/presentation.md examples/output/rust_presentation.pptx --title "Rust Programming"
echo "   ✓ Created: examples/output/rust_presentation.pptx"
echo

echo "2. Generating business presentation..."
cargo run -- from-md examples/business.md examples/output/business_review.pptx --title "Q4 2025 Business Review"
echo "   ✓ Created: examples/output/business_review.pptx"
echo

echo "3. Generating technical presentation..."
cargo run -- from-md examples/technical.md examples/output/web_architecture.pptx --title "Building Scalable Web Applications"
echo "   ✓ Created: examples/output/web_architecture.pptx"
echo

echo "✅ All presentations generated successfully!"
echo
echo "Generated files:"
ls -lh examples/output/*.pptx | awk '{print "   " $9 " (" $5 ")"}'
echo
echo "You can now open these files in PowerPoint, LibreOffice, or Google Slides!"
