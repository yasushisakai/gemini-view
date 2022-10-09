#!/bin/bash
curl -s https://mirage.city/uploads/ | rg -o '[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}_[0-9]*_[0-9a-z_]*.jpg' | sort > remote.txt
find ~/code/gemini-view/assets/images -type f | sed 's/^assets\/images\///'  | sort > local.txt
join -v2 local.txt remote.txt | xargs -I@ curl https://mirage.city/uploads/@ -o ~/code/gemini-view/assets/images/@
rm local.txt remote.txt
