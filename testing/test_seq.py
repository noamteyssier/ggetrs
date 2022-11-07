import ggetrs

def test_seq_dna():
    gene = "AP2S1"
    result = ggetrs.seq([gene])
    assert(len(result) == 1)
    assert(len(result[0].keys()) == 2)
    assert(result[0]["header"] == "ENSG00000042753 chromosome:GRCh38:19:46838136:46850846:-1")
    assert("GGGCCCTACAACTGCACCCTGAGCCGGAGCTGCCCAGTCGCCGCGGGACCGGGGCCGCTGGGGTCTGGACGGGGGTCGCCATGGTAACGGGGGAGCGCTACGCCGGGGACTGGCGGAGGG" in result[0]["sequence"])

def test_seq_protein():
    gene = "AP2S1"
    result = ggetrs.seq([gene], translate=True)
    assert(len(result) == 1)
    assert(len(result[0].keys()) == 2)
    assert(result[0]["header"] == "sp|P53680|AP2S1_HUMAN AP-2 complex subunit sigma OS=Human OX=9606 [GN=AP2S1 ] PE=1 SV=2")
    assert("MIRFILIQNRAGKTRLAKWYMQFDDDEKQKLIEEVHAVVTVRDAKHTNFVEFRNFKIIYRRYAGLYFCICVDVNDNNLAYLEAIHNFVEVLNEYFHNVCELDLVFNFYKVYTVVDEMFLAGEIRETSQTKVLKQLLMLQSLE" == result[0]["sequence"])

def test_seq_multiple():
    genes = ["AP2S1", "RFX3", "NSD1"]
    result = ggetrs.seq(genes)
    assert(len(result) == len(genes))
    assert(len(result[0].keys()) == 2)
    assert(len(result[1].keys()) == 2)
    assert(len(result[2].keys()) == 2)

def test_seq_nolist():
    genes = "AP2S1"
    try:
        ggetrs.seq(genes)
        assert(False)
    except:
        assert(True)
