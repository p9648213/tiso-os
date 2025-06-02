#!/bin/bash

# Run Tailwind CLI in background
nodemon --watch './tailwind.css' --watch './src/**/*.{html,rs}' --ext '*' --exec 'npx @tailwindcss/cli -i ./tailwind.css -o ./assets/css/lib/tailwind.css' &

# Run your Rust app with cargo watch
cargo watch -c -w src -x run