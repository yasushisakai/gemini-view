#!/bin/bash
GEMINIVIEWDIR=/Users/yasushi/code/gemini-view/
curl -s https://mirage.city/uploads/ | grep -oE '[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}_[0-9]*_[0-9a-z_]*.jpg' | sort > $GEMINIVIEWDIR/remote.txt
find $GEMINIVIEWDIR/assets/images -type f | sed 's/^assets\/images\///'  | sort > $GEMINIVIEWDIR/local.txt
join -v2 $GEMINIVIEWDIR/local.txt $GEMINIVIEWDIR/remote.txt | xargs -I@ curl https://mirage.city/uploads/@ -o $GEMINIVIEWDIR/assets/images/@
rm $GEMINIVIEWDIR/local.txt $GEMINIVIEWDIR/remote.txt
