import ggetrs
import requests
ENSEMBL_RELEASE = 109

def ping_ensembl_ftp():
    try:
        requests.head("https://ftp.ensembl.org", timeout = 5)
        return True
    except:
        # ensembl ftp server is down; skip check
        return False

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
    if ping_ensembl_ftp():
        results = ggetrs.ensembl.database(f"homo_sapiens_core_{ENSEMBL_RELEASE}_38")
        assert(isinstance(results, list))
        assert(len(results) == 1)
        assert(results[0] == f"homo_sapiens_core_{ENSEMBL_RELEASE}_38")

def test_release():
    results = ggetrs.ensembl.release()
    assert(results == ENSEMBL_RELEASE)

def test_reference():
    if ping_ensembl_ftp():
        results = ggetrs.ensembl.reference(datatype=["dna"])
        assert(isinstance(results, list))
        assert(len(results) == 1)
        assert(isinstance(results[0], dict))
        assert(results[0]["url"] == f"http://ftp.ensembl.org/pub/release-{ENSEMBL_RELEASE}/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna.primary_assembly.fa.gz")

def test_species():
    if ping_ensembl_ftp():
        results = ggetrs.ensembl.species()
        assert(isinstance(results, list))
        assert(len(results) > 1)
        assert("homo_sapiens" in results)
