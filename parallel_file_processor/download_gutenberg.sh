set -euo pipefail

mkdir -p books

echo "Downloading Gutenberg books..."

count=0
for id in $(seq 1 300); do
  url="https://www.gutenberg.org/cache/epub/${id}/pg${id}.txt"
  out="books/pg${id}.txt"

  if curl -L -sS "$url" -o "$out"; then
    size=$(wc -c < "$out" | tr -d ' ')
    if [ "$size" -gt 2000 ]; then
      echo "✔ Downloaded book $id"
      count=$((count + 1))
    else
      rm -f "$out"
    fi
  else
    rm -f "$out"
  fi

  if [ "$count" -ge 120 ]; then
    break
  fi
done

echo "Done. Downloaded $count books."
