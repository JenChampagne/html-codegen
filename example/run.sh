parallel --ungroup ::: \
    "cd api-rocket; ./watch.sh" \
    "cd html; ./watch.sh" \
    "unbuffer websocat ws://127.0.0.1:3232/"
