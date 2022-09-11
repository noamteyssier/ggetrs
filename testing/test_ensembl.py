import ggetrs

def test_search():
    species = "homo_sapiens"
    db_type = "core"
    release = 107
    assembly = "38"
    search_terms = ["AP2S1"]
    results = ggetrs.search(search_terms, species, db_type, release, assembly)
    assert("results" in results.keys())
    assert(len(results["results"]) == 1)
    assert("display_label" in results["results"][0].keys())
    assert(results["results"][0]["display_label"] == "AP2S1")
