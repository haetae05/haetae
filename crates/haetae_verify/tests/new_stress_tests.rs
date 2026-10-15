#[allow(unused_imports)]
use haetae_core::config::{
    NUM_EIGHT, NUM_FIFTEEN, NUM_FIVE, NUM_FORTY, NUM_FOUR, NUM_NEG_ONE, NUM_ONE, NUM_ONE_THOUSAND,
    NUM_SEVEN, NUM_SIX, NUM_SIXTEEN, NUM_TEN, NUM_THIRTEEN, NUM_THIRTY, NUM_THREE, NUM_TWELVE,
    NUM_TWENTY_FIVE, NUM_TWO, NUM_ZERO, POSIX_SIGFPE, POSIX_SIGSEGV,
};
use haetae_verify::fuzzer::FuzzerEngine;
use haetae_verify::tcc_engine::TccEngine;

// ============================================================================
// Area 1: Graph Algorithms (Dijkstra, BFS, Bellman-Ford, Topological Sort, Cycle, Bipartite)
// ============================================================================

#[test]
fn test_stress_01_graph_dijkstra_shortest_path() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { int d0 = 0; int d1 = 4; int d2 = 2; int d3 = 7; int alt1 = d2 + 1; d1 = (alt1 < d1) ? alt1 : d1; int alt3 = d1 + 2; d3 = (alt3 < d3) ? alt3 : d3; int target = (int)(p & 3); return (target == 0) ? d0 : ((target == 1) ? d1 : ((target == 2) ? d2 : d3)); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile dijkstra");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz dijkstra");
    assert_eq!(res.failed, NUM_ZERO);
    assert_eq!(res.total_iterations, NUM_ONE_THOUSAND);

    let d1_val = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("exec d1");
    assert_eq!(d1_val, NUM_THREE as i32);
    let d2_val = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("exec d2");
    assert_eq!(d2_val, NUM_TWO as i32);
    let d3_val = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("exec d3");
    assert_eq!(d3_val, NUM_FIVE as i32);
}

#[test]
fn test_stress_02_graph_bfs_reachability_layer() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { int dist0 = 0; int dist1 = dist0 + 1; int dist2 = dist0 + 1; int dist3 = dist1 + 1; int target = (int)(p & 3); return (target == 0) ? dist0 : ((target == 3) ? dist3 : dist1); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile bfs");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz bfs");
    assert_eq!(res.failed, NUM_ZERO);

    let dist0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("exec target 0");
    assert_eq!(dist0, NUM_ZERO as i32);
    let dist1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("exec target 1");
    assert_eq!(dist1, NUM_ONE as i32);
    let dist3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("exec target 3");
    assert_eq!(dist3, NUM_TWO as i32);
}

#[test]
fn test_stress_03_graph_bellman_ford_negative_edge() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { int d0 = 0; int d1 = 4; int d2 = 5; int alt2 = d1 - 2; d2 = (alt2 < d2) ? alt2 : d2; int node = (int)(p & 1); return (node == 0) ? d1 : d2; }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile bellman_ford");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz bellman_ford");
    assert_eq!(res.failed, NUM_ZERO);

    let d1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("exec node 1");
    assert_eq!(d1, NUM_FOUR as i32);
    let d2 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("exec node 2");
    assert_eq!(d2, NUM_TWO as i32);
}

#[test]
fn test_stress_04_graph_topological_sort_dag_order() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { int in0 = 0; int in1 = 1; int in2 = 1; int in3 = 2; int node = (int)(p & 3); return (node == 0) ? in0 : ((node == 3) ? in3 : in1); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile topo_sort");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz topo_sort");
    assert_eq!(res.failed, NUM_ZERO);

    let in0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("exec rank 0");
    assert_eq!(in0, NUM_ZERO as i32);
    let in1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("exec rank 1");
    assert_eq!(in1, NUM_ONE as i32);
    let in3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("exec rank 3");
    assert_eq!(in3, NUM_TWO as i32);
}

#[test]
fn test_stress_05_graph_cycle_detection() {
    let tcc = TccEngine::new();
    let code = "int has_cycle(int gid) { if (gid == 0) return 0; return 1; } int haetae_fuzz_entry(long long p) { int gid = (int)(p & 1); return has_cycle(gid); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile cycle_detect");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz cycle_detect");
    assert_eq!(res.failed, NUM_ZERO);

    let c0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("exec acyclic");
    assert_eq!(c0, NUM_ZERO as i32);
    let c1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("exec cyclic");
    assert_eq!(c1, NUM_ONE as i32);
}

#[test]
fn test_stress_06_graph_bipartite_coloring() {
    let tcc = TccEngine::new();
    let code = "int check_bipartite(int nodes) { if (nodes == 4) return 1; return 0; } int haetae_fuzz_entry(long long p) { int nodes = ((int)(p & 1) == 0) ? 4 : 3; return check_bipartite(nodes); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile bipartite");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz bipartite");
    assert_eq!(res.failed, NUM_ZERO);

    let bip_even = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("exec even cycle");
    assert_eq!(bip_even, NUM_ONE as i32);
    let bip_odd = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("exec odd cycle");
    assert_eq!(bip_odd, NUM_ZERO as i32);
}

// ============================================================================
// Area 2: Tree & Heap Algorithms (BST, Max Heap, Trie, Fenwick, LCA, Tree Depth)
// ============================================================================

#[test]
fn test_stress_07_tree_bst_search() {
    let tcc = TccEngine::new();
    let code = "int bst_find(int key) { if (key == 10) return 1; if (key == 5) return 1; if (key == 15) return 1; return 0; } int haetae_fuzz_entry(long long p) { int key = (int)(p & 31); return bst_find(key); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile bst");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz bst");
    assert_eq!(res.failed, NUM_ZERO);

    let f_root = tcc
        .execute_entry_with_payload(&compiled, NUM_TEN as i64)
        .expect("find 10");
    assert_eq!(f_root, NUM_ONE as i32);
    let f_left = tcc
        .execute_entry_with_payload(&compiled, NUM_FIVE as i64)
        .expect("find 5");
    assert_eq!(f_left, NUM_ONE as i32);
    let f_right = tcc
        .execute_entry_with_payload(&compiled, NUM_FIFTEEN as i64)
        .expect("find 15");
    assert_eq!(f_right, NUM_ONE as i32);
    let f_none = tcc
        .execute_entry_with_payload(&compiled, NUM_SEVEN as i64)
        .expect("find 7");
    assert_eq!(f_none, NUM_ZERO as i32);
}

#[test]
fn test_stress_08_heap_max_sift_down() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { int h0 = (int)(p & 7); int h1 = 30; int h2 = 20; int largest = 0; largest = (h1 > h0) ? 1 : largest; int largest_val = (largest == 1) ? h1 : h0; largest = (h2 > largest_val) ? 2 : largest; int top = (largest == 1) ? h1 : ((largest == 2) ? h2 : h0); return top; }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile max_heap");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz max_heap");
    assert_eq!(res.failed, NUM_ZERO);

    let top = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("exec heap");
    assert_eq!(top, NUM_THIRTY as i32);
}

#[test]
fn test_stress_09_tree_trie_prefix_match() {
    let tcc = TccEngine::new();
    let code = "int trie_step(int state, int c) { if (state == 0) return c; if (state == 1) return c + 1; return 0; } int haetae_fuzz_entry(long long p) { int char1 = ((int)(p & 1)) + 1; int s1 = trie_step(0, char1); int s2 = trie_step(s1, 2); return (s2 == 3) ? 1 : 0; }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile trie");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz trie");
    assert_eq!(res.failed, NUM_ZERO);

    let m_ok = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("match ok");
    assert_eq!(m_ok, NUM_ONE as i32);
    let m_no = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("match none");
    assert_eq!(m_no, NUM_ZERO as i32);
}

#[test]
fn test_stress_10_tree_fenwick_prefix_sum() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { int tree[5] = {0, 3, 5, 4, 10}; int idx = (int)(p & 3) + 1; return tree[idx]; }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile fenwick");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz fenwick");
    assert_eq!(res.failed, NUM_ZERO);

    let s1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("query 1");
    assert_eq!(s1, NUM_THREE as i32);
    let s2 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("query 2");
    assert_eq!(s2, NUM_FIVE as i32);
    let s3 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("query 3");
    assert_eq!(s3, NUM_FOUR as i32);
    let s4 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("query 4");
    assert_eq!(s4, NUM_TEN as i32);
}

#[test]
fn test_stress_11_tree_lowest_common_ancestor() {
    let tcc = TccEngine::new();
    let code = "int lca(int u, int v) { if (u == v) return u; if (u == 4 && v == 5) return 2; if (u == 4 && v == 3) return 1; if (u == 2 && v == 4) return 2; return 1; } int haetae_fuzz_entry(long long p) { int u = (int)(p & 3) + 2; int v = 3; return lca(u, v); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile lca");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz lca");
    assert_eq!(res.failed, NUM_ZERO);

    let lca_same = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("lca 3 3");
    assert_eq!(lca_same, NUM_THREE as i32);
    let lca_4_3 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("lca 4 3");
    assert_eq!(lca_4_3, NUM_ONE as i32);
}

#[test]
fn test_stress_12_tree_depth_and_diameter() {
    let tcc = TccEngine::new();
    let code = "int node_depth(int node) { if (node <= 1) return 1; if (node <= 3) return 2; return 3; } int haetae_fuzz_entry(long long p) { int node = (int)(p & 7); return node_depth(node); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile depth");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz depth");
    assert_eq!(res.failed, NUM_ZERO);

    let d0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("depth 0");
    assert_eq!(d0, NUM_ONE as i32);
    let d2 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("depth 2");
    assert_eq!(d2, NUM_TWO as i32);
    let d5 = tcc
        .execute_entry_with_payload(&compiled, NUM_FIVE as i64)
        .expect("depth 5");
    assert_eq!(d5, NUM_THREE as i32);
}

// ============================================================================
// Area 3: Dynamic Programming (0/1 Knapsack, LCS, Matrix Chain, Coin Change, Subset Sum, Rod Cut)
// ============================================================================

#[test]
fn test_stress_13_dp_01_knapsack_optimization() {
    let tcc = TccEngine::new();
    let code = "int knapsack(int w) { if (w <= 0) return 0; if (w == 1) return 6; if (w == 2) return 10; return 16; } int haetae_fuzz_entry(long long p) { int cap = (int)(p & 3); return knapsack(cap); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile knapsack");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz knapsack");
    assert_eq!(res.failed, NUM_ZERO);

    let w0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("w0");
    assert_eq!(w0, NUM_ZERO as i32);
    let w1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("w1");
    assert_eq!(w1, NUM_SIX as i32);
    let w2 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("w2");
    assert_eq!(w2, NUM_TEN as i32);
    let w3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("w3");
    assert_eq!(w3, NUM_SIXTEEN as i32);
}

#[test]
fn test_stress_14_dp_longest_common_subsequence() {
    let tcc = TccEngine::new();
    let code = "int lcs_table(int i, int j) { if (i == 0 || j == 0) return 0; if (i == 1 && j == 1) return 1; if (i == 2 && j == 2) return 2; if (i == 3 && j == 3) return 3; return 4; } int haetae_fuzz_entry(long long p) { int i = (int)(p & 3); return lcs_table(i, i); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile lcs");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz lcs");
    assert_eq!(res.failed, NUM_ZERO);

    let lcs0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("lcs 0");
    assert_eq!(lcs0, NUM_ZERO as i32);
    let lcs1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("lcs 1");
    assert_eq!(lcs1, NUM_ONE as i32);
    let lcs2 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("lcs 2");
    assert_eq!(lcs2, NUM_TWO as i32);
    let lcs3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("lcs 3");
    assert_eq!(lcs3, NUM_THREE as i32);
}

#[test]
fn test_stress_15_dp_matrix_chain_multiplication() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { int cost1 = 2 * 3; int cost2 = 3 * 4; int min_cost = (cost1 < cost2) ? cost1 : cost2; int offset = (int)(p & 1); return min_cost + offset; }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile matrix_chain");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz matrix_chain");
    assert_eq!(res.failed, NUM_ZERO);

    let base_cost = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("base cost");
    assert_eq!(base_cost, NUM_SIX as i32);
    let off_cost = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("off cost");
    assert_eq!(off_cost, NUM_SEVEN as i32);
}

#[test]
fn test_stress_16_dp_coin_change_minimum_coins() {
    let tcc = TccEngine::new();
    let code = "int min_coins(int amount) { if (amount <= 0) return 0; if (amount == 1 || amount == 2 || amount == 5) return 1; return 2; } int haetae_fuzz_entry(long long p) { int amt = (int)(p & 7); return min_coins(amt); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile coin_change");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz coin_change");
    assert_eq!(res.failed, NUM_ZERO);

    let c0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("c0");
    assert_eq!(c0, NUM_ZERO as i32);
    let c1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("c1");
    assert_eq!(c1, NUM_ONE as i32);
    let c2 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("c2");
    assert_eq!(c2, NUM_ONE as i32);
    let c3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("c3");
    assert_eq!(c3, NUM_TWO as i32);
}

#[test]
fn test_stress_17_dp_subset_sum_decision() {
    let tcc = TccEngine::new();
    let code = "int subset_sum_exists(int target) { if (target == 0 || target == 3 || target == 4 || target == 7) return 1; return 0; } int haetae_fuzz_entry(long long p) { int t = (int)(p & 7); return subset_sum_exists(t); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile subset_sum");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz subset_sum");
    assert_eq!(res.failed, NUM_ZERO);

    let s0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("s0");
    assert_eq!(s0, NUM_ONE as i32);
    let s3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("s3");
    assert_eq!(s3, NUM_ONE as i32);
    let s5 = tcc
        .execute_entry_with_payload(&compiled, NUM_FIVE as i64)
        .expect("s5");
    assert_eq!(s5, NUM_ZERO as i32);
}

#[test]
fn test_stress_18_dp_rod_cutting_max_revenue() {
    let tcc = TccEngine::new();
    let code = "int cut_rod(int len) { if (len <= 0) return 0; if (len == 1) return 1; if (len == 2) return 5; if (len == 3) return 8; return 10; } int haetae_fuzz_entry(long long p) { int l = (int)(p & 7); return cut_rod(l); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile rod_cut");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz rod_cut");
    assert_eq!(res.failed, NUM_ZERO);

    let r1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("r1");
    assert_eq!(r1, NUM_ONE as i32);
    let r2 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("r2");
    assert_eq!(r2, NUM_FIVE as i32);
    let r3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("r3");
    assert_eq!(r3, NUM_EIGHT as i32);
    let r4 = tcc
        .execute_entry_with_payload(&compiled, NUM_FOUR as i64)
        .expect("r4");
    assert_eq!(r4, NUM_TEN as i32);
}

// ============================================================================
// Area 4: String Algorithms (KMP, Levenshtein, RLE, Rabin-Karp, Palindrome, Hamming)
// ============================================================================

#[test]
fn test_stress_19_string_kmp_prefix_table() {
    let tcc = TccEngine::new();
    let code = "int kmp_pi(int idx) { if (idx <= 1) return 0; if (idx == 2) return 1; if (idx == 3) return 2; if (idx == 4) return 0; if (idx == 5) return 1; if (idx == 6) return 2; if (idx == 7) return 3; return 4; } int haetae_fuzz_entry(long long p) { int idx = (int)(p & 7); return kmp_pi(idx); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile kmp");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz kmp");
    assert_eq!(res.failed, NUM_ZERO);

    let pi1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("pi 1");
    assert_eq!(pi1, NUM_ZERO as i32);
    let pi2 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("pi 2");
    assert_eq!(pi2, NUM_ONE as i32);
    let pi3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("pi 3");
    assert_eq!(pi3, NUM_TWO as i32);
}

#[test]
fn test_stress_20_string_levenshtein_distance() {
    let tcc = TccEngine::new();
    let code = "int edit_dist(int len1, int len2, int diff) { if (len1 == 0) return len2; if (len2 == 0) return len1; return (diff == 0) ? 0 : 1; } int haetae_fuzz_entry(long long p) { int diff = (int)(p & 1); return edit_dist(3, 3, diff); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile levenshtein");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz levenshtein");
    assert_eq!(res.failed, NUM_ZERO);

    let same = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("same");
    assert_eq!(same, NUM_ZERO as i32);
    let diff = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("diff");
    assert_eq!(diff, NUM_ONE as i32);
}

#[test]
fn test_stress_21_string_run_length_encoding() {
    let tcc = TccEngine::new();
    let code = "int rle_encode_length(int runs) { return runs * 2; } int haetae_fuzz_entry(long long p) { int runs = (int)(p & 7) + 1; return rle_encode_length(runs); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile rle");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz rle");
    assert_eq!(res.failed, NUM_ZERO);

    let rle1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("rle 1 run");
    assert_eq!(rle1, NUM_TWO as i32);
    let rle2 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("rle 2 runs");
    assert_eq!(rle2, NUM_FOUR as i32);
    let rle3 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("rle 3 runs");
    assert_eq!(rle3, NUM_SIX as i32);
}

#[test]
fn test_stress_22_string_rabin_karp_rolling_hash() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { int b0 = (int)(p & 15); int h = 0; h = (h * 31 + b0) % 10007; h = (h * 31 + 65) % 10007; return h; }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile rabin_karp");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz rabin_karp");
    assert_eq!(res.failed, NUM_ZERO);

    let h = tcc
        .execute_entry_with_payload(&compiled, NUM_TEN as i64)
        .expect("hash 10");
    assert!(h > NUM_ZERO as i32);
}

#[test]
fn test_stress_23_string_palindrome_radius() {
    let tcc = TccEngine::new();
    let code = "int palindrome_radius(int left_c, int right_c) { if (left_c == right_c) return 2; return 1; } int haetae_fuzz_entry(long long p) { int c1 = (int)(p & 1); int c2 = (int)(p & 1); return palindrome_radius(c1, c2); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile palindrome");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz palindrome");
    assert_eq!(res.failed, NUM_ZERO);

    let rad_eq = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("rad equal");
    assert_eq!(rad_eq, NUM_TWO as i32);
}

#[test]
fn test_stress_24_string_hamming_distance() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { int b0 = (int)(p & 1); int b1 = (int)((p >> 1) & 1); int dist = (b0 != 1) ? 1 : 0; dist = dist + ((b1 != 1) ? 1 : 0); return dist; }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile hamming");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz hamming");
    assert_eq!(res.failed, NUM_ZERO);

    let d0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("dist 0");
    assert_eq!(d0, NUM_TWO as i32);
    let d3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("dist 3");
    assert_eq!(d3, NUM_ZERO as i32);
}

// ============================================================================
// Area 5: Bit Manipulation (Popcount, Parity, Reverse, Gray Code, ModExp, Lowest Bit)
// ============================================================================

#[test]
fn test_stress_25_bit_hamming_weight_popcount() {
    let tcc = TccEngine::new();
    let code = "int popcount4(int x) { int c = 0; c = c + (x & 1); x = x >> 1; c = c + (x & 1); x = x >> 1; c = c + (x & 1); x = x >> 1; c = c + (x & 1); return c; } int haetae_fuzz_entry(long long p) { int val = (int)(p & 15); return popcount4(val); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile popcount");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz popcount");
    assert_eq!(res.failed, NUM_ZERO);

    let p0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("pop 0");
    assert_eq!(p0, NUM_ZERO as i32);
    let p1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("pop 1");
    assert_eq!(p1, NUM_ONE as i32);
    let p3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("pop 3");
    assert_eq!(p3, NUM_TWO as i32);
    let p7 = tcc
        .execute_entry_with_payload(&compiled, NUM_SEVEN as i64)
        .expect("pop 7");
    assert_eq!(p7, NUM_THREE as i32);
    let p15 = tcc
        .execute_entry_with_payload(&compiled, NUM_FIFTEEN as i64)
        .expect("pop 15");
    assert_eq!(p15, NUM_FOUR as i32);
}

#[test]
fn test_stress_26_bit_parity_calculation() {
    let tcc = TccEngine::new();
    let code = "int calc_parity(int x) { int p = x ^ (x >> 1); p = p ^ (p >> 2); return p & 1; } int haetae_fuzz_entry(long long p) { int val = (int)(p & 15); return calc_parity(val); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile parity");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz parity");
    assert_eq!(res.failed, NUM_ZERO);

    let par0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("par 0");
    assert_eq!(par0, NUM_ZERO as i32);
    let par1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("par 1");
    assert_eq!(par1, NUM_ONE as i32);
    let par3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("par 3");
    assert_eq!(par3, NUM_ZERO as i32);
    let par7 = tcc
        .execute_entry_with_payload(&compiled, NUM_SEVEN as i64)
        .expect("par 7");
    assert_eq!(par7, NUM_ONE as i32);
}

#[test]
fn test_stress_27_bit_reverse_nibbles() {
    let tcc = TccEngine::new();
    let code = "int reverse_nibble(int x) { int b0 = (x & 1) << 3; int b1 = (x & 2) << 1; int b2 = (x & 4) >> 1; int b3 = (x & 8) >> 3; return b0 | b1 | b2 | b3; } int haetae_fuzz_entry(long long p) { int val = (int)(p & 15); return reverse_nibble(val); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile rev_nibble");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz rev_nibble");
    assert_eq!(res.failed, NUM_ZERO);

    let rev1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("rev 1");
    assert_eq!(rev1, NUM_EIGHT as i32);
    let rev8 = tcc
        .execute_entry_with_payload(&compiled, NUM_EIGHT as i64)
        .expect("rev 8");
    assert_eq!(rev8, NUM_ONE as i32);
}

#[test]
fn test_stress_28_bit_gray_code_roundtrip() {
    let tcc = TccEngine::new();
    let code = "int bin_to_gray(int n) { return n ^ (n >> 1); } int gray_to_bin(int g) { int b = g ^ (g >> 1); b = b ^ (b >> 2); return b; } int haetae_fuzz_entry(long long p) { int n = (int)(p & 7); int g = bin_to_gray(n); return gray_to_bin(g); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile gray");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz gray");
    assert_eq!(res.failed, NUM_ZERO);

    let rt0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("gray 0");
    assert_eq!(rt0, NUM_ZERO as i32);
    let rt3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("gray 3");
    assert_eq!(rt3, NUM_THREE as i32);
    let rt5 = tcc
        .execute_entry_with_payload(&compiled, NUM_FIVE as i64)
        .expect("gray 5");
    assert_eq!(rt5, NUM_FIVE as i32);
}

#[test]
fn test_stress_29_bit_fast_modular_exponentiation() {
    let tcc = TccEngine::new();
    let code = "int mod_exp(int base, int exp, int mod) { if (exp <= 0) return 1; if (exp == 1) return base % mod; int half = mod_exp(base, exp / 2, mod); int sq = (half * half) % mod; return (exp % 2 == 1) ? ((sq * base) % mod) : sq; } int haetae_fuzz_entry(long long p) { int e = (int)(p & 7); return mod_exp(2, e, 10); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile mod_exp");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz mod_exp");
    assert_eq!(res.failed, NUM_ZERO);

    let pow3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("2^3 % 10");
    assert_eq!(pow3, NUM_EIGHT as i32);
    let pow4 = tcc
        .execute_entry_with_payload(&compiled, NUM_FOUR as i64)
        .expect("2^4 % 10");
    assert_eq!(pow4, NUM_SIX as i32);
}

#[test]
fn test_stress_30_bit_isolate_lowest_set_bit() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { int x = ((int)(p & 15)) + 1; int lowest = x & (-x); int cleared = x & (x - 1); return (cleared + lowest == x) ? 1 : 0; }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile lowest_bit");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz lowest_bit");
    assert_eq!(res.failed, NUM_ZERO);

    let id1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("check identity 1");
    assert_eq!(id1, NUM_ONE as i32);
    let id2 = tcc
        .execute_entry_with_payload(&compiled, NUM_FIVE as i64)
        .expect("check identity 5");
    assert_eq!(id2, NUM_ONE as i32);
}

// ============================================================================
// Area 6: Sorting & Searching (Binary Search, Quickselect, Merge, Counting, Insertion, Ternary)
// ============================================================================

#[test]
fn test_stress_31_sort_binary_search_exact() {
    let tcc = TccEngine::new();
    let code = "int bsearch_arr(int target) { if (target == 2) return 0; if (target == 5) return 1; if (target == 8) return 2; if (target == 12) return 3; return -1; } int haetae_fuzz_entry(long long p) { int t = (int)(p & 15); return bsearch_arr(t); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile bsearch");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz bsearch");
    assert_eq!(res.failed, NUM_ZERO);

    let f0 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("find 2");
    assert_eq!(f0, NUM_ZERO as i32);
    let f1 = tcc
        .execute_entry_with_payload(&compiled, NUM_FIVE as i64)
        .expect("find 5");
    assert_eq!(f1, NUM_ONE as i32);
    let f2 = tcc
        .execute_entry_with_payload(&compiled, NUM_EIGHT as i64)
        .expect("find 8");
    assert_eq!(f2, NUM_TWO as i32);
    let f_none = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("find 1");
    assert_eq!(f_none, NUM_NEG_ONE as i32);
}

#[test]
fn test_stress_32_sort_quickselect_kth_element() {
    let tcc = TccEngine::new();
    let code = "int kth_smallest(int k) { if (k <= 0) return 1; if (k == 1) return 2; if (k == 2) return 3; return 5; } int haetae_fuzz_entry(long long p) { int k = (int)(p & 3); return kth_smallest(k); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile quickselect");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz quickselect");
    assert_eq!(res.failed, NUM_ZERO);

    let k0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("k0");
    assert_eq!(k0, NUM_ONE as i32);
    let k1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("k1");
    assert_eq!(k1, NUM_TWO as i32);
    let k2 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("k2");
    assert_eq!(k2, NUM_THREE as i32);
    let k3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("k3");
    assert_eq!(k3, NUM_FIVE as i32);
}

#[test]
fn test_stress_33_sort_merge_sorted_arrays() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { int merged[6] = {1, 2, 4, 5, 8, 9}; int idx = (int)(p & 3); return merged[idx]; }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile merge");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz merge");
    assert_eq!(res.failed, NUM_ZERO);

    let m0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("m0");
    assert_eq!(m0, NUM_ONE as i32);
    let m1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("m1");
    assert_eq!(m1, NUM_TWO as i32);
    let m2 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("m2");
    assert_eq!(m2, NUM_FOUR as i32);
    let m3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("m3");
    assert_eq!(m3, NUM_FIVE as i32);
}

#[test]
fn test_stress_34_sort_counting_frequency_sort() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { int sorted[4] = {0, 1, 2, 4}; int idx = (int)(p & 3); return sorted[idx]; }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile counting_sort");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz counting_sort");
    assert_eq!(res.failed, NUM_ZERO);

    let s0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("s0");
    assert_eq!(s0, NUM_ZERO as i32);
    let s1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("s1");
    assert_eq!(s1, NUM_ONE as i32);
    let s2 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("s2");
    assert_eq!(s2, NUM_TWO as i32);
    let s3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("s3");
    assert_eq!(s3, NUM_FOUR as i32);
}

#[test]
fn test_stress_35_sort_insertion_order_swap() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { int a0 = 40; int a1 = 10; int t0 = (a0 < a1) ? a0 : a1; int t1 = (a0 < a1) ? a1 : a0; int idx = (int)(p & 1); return (idx == 0) ? t0 : t1; }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile insertion");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz insertion");
    assert_eq!(res.failed, NUM_ZERO);

    let i0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("i0");
    assert_eq!(i0, NUM_TEN as i32);
    let i1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("i1");
    assert_eq!(i1, NUM_FORTY as i32);
}

#[test]
fn test_stress_36_search_ternary_peak_detection() {
    let tcc = TccEngine::new();
    let code = "int eval_parabola(int x) { int diff = x - 4; return 25 - diff * diff; } int haetae_fuzz_entry(long long p) { int x = (int)(p & 7); return eval_parabola(x); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile ternary_search");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz ternary_search");
    assert_eq!(res.failed, NUM_ZERO);

    let peak = tcc
        .execute_entry_with_payload(&compiled, NUM_FOUR as i64)
        .expect("peak at 4");
    assert_eq!(peak, NUM_TWENTY_FIVE as i32);
}

// ============================================================================
// Area 7: Mathematical Algorithms (GCD, LCM, Sieve, Collatz, Newton, Fib, Catalan)
// ============================================================================

#[test]
fn test_stress_37_math_gcd_euclidean() {
    let tcc = TccEngine::new();
    let code = "int gcd(int a, int b) { if (b <= 0) return a; return gcd(b, a % b); } int haetae_fuzz_entry(long long p) { int b = ((int)(p & 7)) + 1; return gcd(24, b); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile gcd");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz gcd");
    assert_eq!(res.failed, NUM_ZERO);

    let gcd_val = tcc
        .execute_entry_with_payload(&compiled, NUM_SEVEN as i64)
        .expect("gcd(24, 8)");
    assert_eq!(gcd_val, NUM_EIGHT as i32);
    let gcd_6 = tcc
        .execute_entry_with_payload(&compiled, NUM_FIVE as i64)
        .expect("gcd(24, 6)");
    assert_eq!(gcd_6, NUM_SIX as i32);
}

#[test]
fn test_stress_38_math_lcm_computation() {
    let tcc = TccEngine::new();
    let code = "int gcd_calc(int a, int b) { if (b <= 0) return a; return gcd_calc(b, a % b); } int lcm_calc(int a, int b) { int g = gcd_calc(a, b); return (a * b) / g; } int haetae_fuzz_entry(long long p) { int b = ((int)(p & 3)) + 1; return lcm_calc(6, b); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile lcm");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz lcm");
    assert_eq!(res.failed, NUM_ZERO);

    let lcm_3 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("lcm(6, 3)");
    assert_eq!(lcm_3, NUM_SIX as i32);
    let lcm_4 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("lcm(6, 4)");
    assert_eq!(lcm_4, NUM_TWELVE as i32);
}

#[test]
fn test_stress_39_math_sieve_primes_validation() {
    let tcc = TccEngine::new();
    let code = "int is_prime(int n) { if (n < 2) return 0; if (n == 2 || n == 3 || n == 5 || n == 7 || n == 11) return 1; return 0; } int haetae_fuzz_entry(long long p) { int n = (int)(p & 15); return is_prime(n); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile sieve");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz sieve");
    assert_eq!(res.failed, NUM_ZERO);

    let p2 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("prime 2");
    assert_eq!(p2, NUM_ONE as i32);
    let p3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("prime 3");
    assert_eq!(p3, NUM_ONE as i32);
    let p4 = tcc
        .execute_entry_with_payload(&compiled, NUM_FOUR as i64)
        .expect("prime 4");
    assert_eq!(p4, NUM_ZERO as i32);
    let p5 = tcc
        .execute_entry_with_payload(&compiled, NUM_FIVE as i64)
        .expect("prime 5");
    assert_eq!(p5, NUM_ONE as i32);
}

#[test]
fn test_stress_40_math_collatz_conjecture_steps() {
    let tcc = TccEngine::new();
    let code = "int collatz_step(int n) { if (n <= 1) return 0; if (n == 2) return 1; if (n == 4) return 2; return 7; } int haetae_fuzz_entry(long long p) { int n = ((int)(p & 3)) + 1; return collatz_step(n); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile collatz");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz collatz");
    assert_eq!(res.failed, NUM_ZERO);

    let c1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("collatz 1");
    assert_eq!(c1, NUM_ZERO as i32);
    let c2 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("collatz 2");
    assert_eq!(c2, NUM_ONE as i32);
    let c4 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("collatz 4");
    assert_eq!(c4, NUM_TWO as i32);
}

#[test]
fn test_stress_41_math_newton_raphson_sqrt() {
    let tcc = TccEngine::new();
    let code = "int newton_sqrt(int n) { if (n <= 0) return 0; if (n < 4) return 1; if (n < 9) return 2; if (n < 16) return 3; if (n < 25) return 4; return 5; } int haetae_fuzz_entry(long long p) { int n = ((int)(p & 15)) + 1; return newton_sqrt(n); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile isqrt");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz isqrt");
    assert_eq!(res.failed, NUM_ZERO);

    let sq9 = tcc
        .execute_entry_with_payload(&compiled, NUM_EIGHT as i64)
        .expect("sqrt 9");
    assert_eq!(sq9, NUM_THREE as i32);
    let sq16 = tcc
        .execute_entry_with_payload(&compiled, NUM_FIFTEEN as i64)
        .expect("sqrt 16");
    assert_eq!(sq16, NUM_FOUR as i32);
}

#[test]
fn test_stress_42_math_fibonacci_recurrence() {
    let tcc = TccEngine::new();
    let code = "int fib_tab(int n) { int f[8] = {0, 1, 1, 2, 3, 5, 8, 13}; return f[(n & 7)]; } int haetae_fuzz_entry(long long p) { int n = (int)(p & 7); return fib_tab(n); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile fib");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz fib");
    assert_eq!(res.failed, NUM_ZERO);

    let f0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("f0");
    assert_eq!(f0, NUM_ZERO as i32);
    let f1 = tcc
        .execute_entry_with_payload(&compiled, NUM_ONE as i64)
        .expect("f1");
    assert_eq!(f1, NUM_ONE as i32);
    let f4 = tcc
        .execute_entry_with_payload(&compiled, NUM_FOUR as i64)
        .expect("f4");
    assert_eq!(f4, NUM_THREE as i32);
    let f7 = tcc
        .execute_entry_with_payload(&compiled, NUM_SEVEN as i64)
        .expect("f7");
    assert_eq!(f7, NUM_THIRTEEN as i32);
}

#[test]
fn test_stress_43_math_catalan_numbers() {
    let tcc = TccEngine::new();
    let code = "int catalan(int n) { if (n <= 0) return 1; if (n == 1) return 1; if (n == 2) return 2; if (n == 3) return 5; return 14; } int haetae_fuzz_entry(long long p) { int n = (int)(p & 3); return catalan(n); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile catalan");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer.run_fuzz_loop(&tcc, &compiled).expect("fuzz catalan");
    assert_eq!(res.failed, NUM_ZERO);

    let cat0 = tcc
        .execute_entry_with_payload(&compiled, NUM_ZERO as i64)
        .expect("cat 0");
    assert_eq!(cat0, NUM_ONE as i32);
    let cat2 = tcc
        .execute_entry_with_payload(&compiled, NUM_TWO as i64)
        .expect("cat 2");
    assert_eq!(cat2, NUM_TWO as i32);
    let cat3 = tcc
        .execute_entry_with_payload(&compiled, NUM_THREE as i64)
        .expect("cat 3");
    assert_eq!(cat3, NUM_FIVE as i32);
}

// ============================================================================
// Area 8: Edge Cases & Defect Detection (Null Deref, Out-of-bounds, Div0, Mod0, Recursion, Stack)
// ============================================================================

#[test]
fn test_stress_44_edge_null_pointer_dereference_defect() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { (void)p; int* ptr = 0; return *ptr; }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile null deref");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz null deref");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(POSIX_SIGSEGV));
}

#[test]
fn test_stress_45_edge_array_upper_bound_defect() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { (void)p; int arr[3] = {10, 20, 30}; return arr[10]; }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile bounds overflow");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz bounds overflow");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(POSIX_SIGSEGV));
}

#[test]
fn test_stress_46_edge_array_negative_bound_defect() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { (void)p; int arr[4] = {1, 2, 3, 4}; int idx = 0 - 1; return arr[idx]; }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile negative idx");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz negative idx");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(POSIX_SIGSEGV));
}

#[test]
fn test_stress_47_edge_division_by_zero_defect() {
    let tcc = TccEngine::new();
    let code = "int haetae_fuzz_entry(long long p) { return (int)(100 / (int)p); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile div zero");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz div zero");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(POSIX_SIGFPE));
}

#[test]
fn test_stress_48_edge_modulo_by_zero_defect() {
    let tcc = TccEngine::new();
    let code =
        "int haetae_fuzz_entry(long long p) { int d = (int)(p & 1); return (int)(100 % d); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile mod zero");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz mod zero");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(POSIX_SIGFPE));
}

#[test]
fn test_stress_49_edge_unbounded_recursion_defect() {
    let tcc = TccEngine::new();
    let code = "int recurse_forever(int x) { return recurse_forever(x + 1); } int haetae_fuzz_entry(long long p) { return recurse_forever((int)p); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile recurse forever");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz recurse forever");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(POSIX_SIGSEGV));
}

#[test]
fn test_stress_50_edge_massive_stack_overflow_defect() {
    let tcc = TccEngine::new();
    let code =
        "int haetae_fuzz_entry(long long p) { char massive_buf[2500000]; return (int)(p & 1); }";
    let compiled = tcc
        .compile_in_memory(code, "haetae_fuzz_entry")
        .expect("compile stack overflow");
    let fuzzer = FuzzerEngine::new(NUM_ONE_THOUSAND);
    let res = fuzzer
        .run_fuzz_loop(&tcc, &compiled)
        .expect("fuzz stack overflow");
    assert_eq!(res.failed, NUM_ONE);
    assert_eq!(res.crash_signal, Some(POSIX_SIGSEGV));
}
