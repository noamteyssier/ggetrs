pub fn shorthand(library: &str) -> String {
    match library {
        "pathway" => "KEGG_2021_Human",
        "transcription" => "ChEA_2016",
        "ontology" => "GO_Biological_Process_2021",
        "diseases_drugs" => "GWAS_Catalog_2019",
        "celltypes" => "PanglaoDB_Augmented_2021",
        "kinase_interactions" => "KEA_2015",
        x => x,
    }
    .to_string()
}

#[cfg(test)]
mod testing {
    use super::shorthand;

    #[test]
    fn test_shorthand_pathway() {
        let lib = "pathway";
        assert_eq!(shorthand(lib), "KEGG_2021_Human".to_string());
    }

    #[test]
    fn test_shorthand_transcription() {
        let lib = "transcription";
        assert_eq!(shorthand(lib), "ChEA_2016".to_string());
    }

    #[test]
    fn test_shorthand_ontology() {
        let lib = "ontology";
        assert_eq!(shorthand(lib), "GO_Biological_Process_2021".to_string());
    }

    #[test]
    fn test_shorthand_diseases() {
        let lib = "diseases_drugs";
        assert_eq!(shorthand(lib), "GWAS_Catalog_2019".to_string());
    }

    #[test]
    fn test_shorthand_celltypes() {
        let lib = "celltypes";
        assert_eq!(shorthand(lib), "PanglaoDB_Augmented_2021".to_string());
    }

    #[test]
    fn test_shorthand_kinase() {
        let lib = "kinase_interactions";
        assert_eq!(shorthand(lib), "KEA_2015".to_string());
    }

    #[test]
    fn test_shorthand_unknown() {
        let lib = "some_random_lib";
        assert_eq!(shorthand(lib), lib.to_string());
    }
}
