import ggetrs

def test_enrichr():
    library = "GO_Biological_Process_2021"
    genes = ["AP2S1", "NSD1", "RFX3"]
    results = ggetrs.enrichr(library, genes)
    assert(type(results) == dict)
    assert(library in results.keys())

