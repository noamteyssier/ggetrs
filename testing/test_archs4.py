import ggetrs

def test_archs4_correlation():
    gene = "AP2S1"
    results = ggetrs.archs4(gene, 5)
    assert(type(results) == dict)
    assert("correlations" in results.keys())
    assert(len(results["correlations"]) == 5)

