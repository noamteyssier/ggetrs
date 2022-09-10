import ggetrs

def test_archs4_correlation():
    gene = "AP2S1"
    results = ggetrs.archs4.correlate(gene, 5)
    assert(type(results) == dict)
    assert("correlations" in results.keys())
    assert(len(results["correlations"]) == 5)

def test_archs4_tissue():
    gene = "AP2S1"
    results = ggetrs.archs4.tissue(gene, "human")
    assert(type(results) == dict)
    assert("tissues" in results.keys())
    assert(len(results["tissues"]) > 0)

