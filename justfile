# all progs
all := "batax_v0_esat batax_v0 batax_v7_csr_dense_unfused_esat batax_v7_csr_dense_unfused mmm_sum_v0_esat mmm_sum_v0 mmm_sum_v7_csc_csr_unfused_esat mmm_sum_v7_csc_csr_unfused mmm_v0_esat mmm_v0 mmm_v7_csr_csr_unfused_esat mmm_v7_csr_csr_unfused mttkrp_v0_esat mttkrp_v0 mttkrp_v7_csf_csr_csc_unfused_esat mttkrp_v7_csf_csr_csc_unfused ttm_v0_esat ttm_v0 ttm_v1_csf_csr_unfused_esat ttm_v1_csf_csr_unfused"

# progs faster than 5s
quick := "batax_v0_esat batax_v7_csr_dense_unfused_esat mmm_sum_v0_esat mmm_sum_v0 mmm_sum_v7_csc_csr_unfused_esat mmm_sum_v7_csc_csr_unfused mmm_v0_esat mmm_v0 mmm_v7_csr_csr_unfused_esat mttkrp_v0_esat mttkrp_v0 mttkrp_v7_csf_csr_csc_unfused_esat ttm_v0_esat ttm_v0 ttm_v1_csf_csr_unfused_esat"



run prog:
    cargo run --release {{prog}}

bench_quick:
    just _bench "{{quick}}"
    
_bench progs:
    #!/usr/bin/env bash
    results="results-$(date --iso-8601=s).csv"
    echo "prog,time" >> "$results"
    for p in {{progs}}; do
        printf "$p," >> "$results" 
        cmd="RUSTFLAGS=-Awarnings just run progs/$p.sexp | grep Total\ time: | grep -Eo '[0-9]+\.[0-9]+' | tr -d '\n' >> $results"
        timeout 20 sh -c "$cmd"
        printf '\n' >> "$results"
    done
    true
    xsv table "$results"
    just sum "$results"

sum file:
    xsv stats {{file}} | xsv select sum
