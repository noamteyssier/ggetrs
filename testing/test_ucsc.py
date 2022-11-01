import ggetrs

def test_ucsc_blat_example_dna():
    sequence = "AGTGGTACATGCAGTTTGATGATGATGAGAAACAGAAGCT"
    response = ggetrs.ucsc.blat(sequence)
    assert(len(response) == 3)
    assert(response[0]["matches"] == 40)

def test_ucsc_blat_example_protein():
    sequence = "MIRFILIQNRAGKTRLAKWYMQFDDDEKQK"
    response = ggetrs.ucsc.blat(sequence, "protein")
    assert(len(response) == 1)
    assert(response[0]["matches"] == 29)

def test_ucsc_blat_missing_db():
    sequence = "AGTGGTACATGCAGTTTGATGATGATGAGAAACAGAAGCT"
    response = ggetrs.ucsc.blat(sequence, db_name="blahblahblah")
    assert(len(response) == 3)
    assert(response[0]["matches"] == 40)

def test_ucsc_blat_example_wrong_seqtype():
    sequence = "MIRFILIQNRAGKTRLAKWYMQFDDDEKQK"
    try:
        response = ggetrs.ucsc.blat(sequence, "dna")
        assert(False)
    except:
        assert(True)
