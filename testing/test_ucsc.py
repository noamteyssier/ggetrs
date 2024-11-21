import ggetrs
import pytest


def test_ucsc_blat_example_dna():
    sequence = "AGTGGTACATGCAGTTTGATGATGATGAGAAACAGAAGCT"
    response = ggetrs.ucsc.blat(sequence)
    assert response[0]["matches"] == 40


def test_ucsc_blat_example_protein():
    sequence = "MIRFILIQNRAGKTRLAKWYMQFDDDEKQK"
    response = ggetrs.ucsc.blat(sequence, "protein")
    assert len(response) == 1
    assert response[0]["matches"] == 29


def test_ucsc_blat_missing_db():
    sequence = "AGTGGTACATGCAGTTTGATGATGATGAGAAACAGAAGCT"
    db_name = "blahblahblah"

    # Use pytest to check that an exception is raised
    with pytest.raises(Exception) as exc_info:
        response = ggetrs.ucsc.blat(sequence, db_name=db_name)

    # Assert that the exception message matches the expected error message
    assert (
        str(exc_info.value)
        == f"Bad response from UCSC Genome Browser. Check Database Name: {db_name}"
    )


def test_ucsc_blat_example_wrong_seqtype():
    sequence = "MIRFILIQNRAGKTRLAKWYMQFDDDEKQK"
    try:
        response = ggetrs.ucsc.blat(sequence, "dna")
        assert False
    except:
        assert True
