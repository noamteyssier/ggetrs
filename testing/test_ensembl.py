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

def test_reference():
    results = ggetrs.ensembl.reference(datatype=["dna"])
    assert(isinstance(results, list))
    assert(len(results) == 1)
    assert(isinstance(results[0], dict))
    assert(results[0]["url"] == "http://ftp.ensembl.org/pub/release-107/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna.primary_assembly.fa.gz")

def test_species():
    results = ggetrs.ensembl.species()
    assert(isinstance(results, list))
    assert(len(results) > 1)
    assert("homo_sapiens" in results)
