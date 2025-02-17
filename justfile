
run prog:
    cargo run --release {{prog}}

bench:
    #!/usr/bin/env bash
    results="results-$(date --iso-8601=s).csv"
    echo "prog,time" >> "$results"
    for p in progs/*; do
        printf "$(basename $p .sexp)," >> "$results" 
        cmd="RUSTFLAGS=-Awarnings just run $p | grep Total\ time: | grep -Eo '[0-9]+\.[0-9]+' | tr -d '\n' >> $results"
        timeout 3 sh -c "$cmd"
        printf '\n' >> "$results"
    done
    true
