#!/usr/bin/env bash
# Run the Holberton project scraper on a link to a Holberton project.
#   The first argument provided to the script is expected to be a
#+  link to a Holberton School project.

PROJECT="$1"
python3 /home/s3kshun-8/GitHub/hipposcraper/hippoproject.py "$PROJECT"
python3 /home/s3kshun-8/GitHub/hipposcraper/hipporead.py "$PROJECT"
