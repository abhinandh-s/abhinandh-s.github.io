dev:
  nix develop

serve:
  taskset -c 0,1 trunk serve

ship:
   git add -A && git commit -m "migration" && git push
