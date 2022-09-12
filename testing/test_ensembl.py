import ggetrs

def test_search_toplevel():
    search_terms = ["AP2S1"]
    results = ggetrs.search(search_terms)
    assert("results" in results.keys())
    assert(len(results["results"]) == 1)
    assert("display_label" in results["results"][0].keys())
    assert(results["results"][0]["display_label"] == "AP2S1")

def test_search_module():
    search_terms = ["AP2S1"]
    results = ggetrs.ensembl.search(search_terms)
    assert("results" in results.keys())
    assert(len(results["results"]) == 1)
    assert("display_label" in results["results"][0].keys())
    assert(results["results"][0]["display_label"] == "AP2S1")

def test_database_nofilter():
    results = ggetrs.ensembl.database()
    assert(isinstance(results, list))
    assert(len(results) > 1)

def test_database_filter():
    results = ggetrs.ensembl.database("homo_sapiens_core_107_38")
    assert(isinstance(results, list))
    assert(len(results) == 1)
    assert(results[0] == "homo_sapiens_core_107_38")

def test_release():
    results = ggetrs.ensembl.release()
    assert(results == 107)
