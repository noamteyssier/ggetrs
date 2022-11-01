import ggetrs

def test_blast():
    sequence = "ATACTCAGTCACACAAGCCATAGCAGGAAACAGCGAGCTTGCAGCCTCACCGACGAGTCTCAACTAAAAGGGACTCCCGGAGCTAGGGGTGGGGACTCGGCCTCACACAGTGAGTGCCGG"
    result = ggetrs.blast(sequence, megablast = False)
    assert result["query"] == sequence
    assert len(result["results"]) >= 1
    assert result["results"][0]["id"] == "gi|2310188890|ref|NG_029005.2|"
