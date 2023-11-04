#!/bin/bash

run_setup() {
  rm -rf .git
  git init

  poetry install

  npm ci
}

starter_name="savaksjcsnciamcacas"
tool_versions_file=".tool-versions"

while true; do
  read -p "Enter the app name (Ctrl+C to exit): " app_name </dev/tty

  if [ -z "$app_name" ]; then
    echo "App name not provided. Please try again."
  else
    break
  fi
done

git clone "https://github.com/Andranik-Solicy/$starter_name.git" "$app_name"
cd "$app_name" || exit

if [ -f "$tool_versions_file" ]; then
  while IFS= read -r line; do
    tool_name=$(echo "$line" | awk '{print $1}')
    tool_version=$(echo "$line" | awk '{print $2}')

    if ! asdf list "$tool_name" | grep -q "$tool_version"; then
      echo "Required $tool_name version $tool_version is not installed."
    fi
  done <"$tool_versions_file"

  run_setup
else
  echo "Error: $tool_versions_file not found"
fi
