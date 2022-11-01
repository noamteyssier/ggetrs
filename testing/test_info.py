import ggetrs

def test_info_single():
    genes = ["AP2S1"]
    results = ggetrs.info(genes)
    assert(len(results.keys()) == 1)
    assert(genes[0] in results.keys())
    assert(results[genes[0]]["pdb_id"] == "6URI")

def test_info_multi():
    genes = ["AP2S1", "NSD1", "RFX3"]
    results = ggetrs.info(genes)
    assert(len(results.keys()) == 3)
    for g in genes:
        assert(g in results.keys())

def test_info_nolist():
    genes = "AP2S1"
    try:
        ggetrs.info(genes)
        assert(False)
    except:
        assert(True)
