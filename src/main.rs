use clap::{CommandFactory, Parser};
use ggetrs::{
    archs4::{launch_archs4_correlation, launch_archs4_tissue},
    blast::cli::launch_blast,
    chembl::launch_chembl_activity,
    cli::{
        Cli, Commands, ModArchS4, ModChembl, ModEnrichr, ModEnsembl, ModNcbi, ModPdb, ModUcsc,
        ModUniprot,
    },
    enrichr::{launch_enrichr, launch_enrichr_list},
    ensembl::{
        launch_ensembl_database, launch_ensembl_list_species, launch_ensembl_lookup_id,
        launch_ensembl_lookup_symbol, launch_ensembl_reference, launch_ensembl_release,
        launch_ensembl_search,
    },
    info::launch_info,
    ncbi::{launch_ncbi_query_ids, launch_ncbi_query_symbols, launch_ncbi_taxons},
    pdb::{launch_pdb_resource, launch_pdb_structure},
    seq::launch_seq,
    ucsc::launch_ucsc_blat,
    uniprot::launch_uniprot_query,
    utils::autocomplete::print_completions,
    RequestError,
};

fn main() -> Result<(), RequestError> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Enrichr(sub) => match sub {
            ModEnrichr::Enrichr {
                library,
                gene_list,
                output,
            } => {
                launch_enrichr(library, gene_list, output)?;
            }
            ModEnrichr::List {
                minimal,
                list_categories,
                category,
                output,
            } => {
                launch_enrichr_list(*minimal, *list_categories, category, output)?;
            }
        },
        Commands::ARCHS4(sub) => match sub {
            ModArchS4::Correlate {
                gene_name,
                count,
                output,
            } => {
                launch_archs4_correlation(gene_name, *count, output)?;
            }
            ModArchS4::Tissue {
                gene_name,
                species,
                output,
            } => {
                launch_archs4_tissue(gene_name, species, output)?;
            }
        },
        Commands::Chembl(sub) => match sub {
            ModChembl::Activity {
                query,
                limit,
                output,
            } => {
                launch_chembl_activity(query, *limit, output)?;
            }
        },
        Commands::Search {
            search_terms,
            database,
            species,
            db_type,
            release,
            assembly,
            output,
        } => {
            launch_ensembl_search(
                search_terms,
                database,
                species,
                db_type,
                release,
                assembly,
                output,
            )?;
        }
        Commands::Info {
            search_terms,
            species,
            taxon_id,
            output,
        } => {
            launch_info(search_terms, species, *taxon_id, output)?;
        }
        Commands::Ensembl(sub) => match sub {
            ModEnsembl::Search {
                search_terms,
                database,
                species,
                db_type,
                release,
                assembly,
                output,
            } => {
                launch_ensembl_search(
                    search_terms,
                    database,
                    species,
                    db_type,
                    release,
                    assembly,
                    output,
                )?;
            }
            ModEnsembl::Database { filter, output } => {
                launch_ensembl_database(filter, output)?;
            }
            ModEnsembl::LookupId {
                ensembl_ids,
                names,
                output,
            } => {
                launch_ensembl_lookup_id(ensembl_ids, *names, output)?;
            }
            ModEnsembl::LookupSymbol {
                symbols,
                species,
                ids,
                output,
            } => {
                launch_ensembl_lookup_symbol(symbols, species, *ids, output)?;
            }
            ModEnsembl::Release => {
                launch_ensembl_release()?;
            }
            ModEnsembl::Ref {
                species,
                release,
                datatype,
                download,
                output,
            } => {
                launch_ensembl_reference(species, *release, datatype, *download, output)?;
            }
            ModEnsembl::Species {
                release,
                datatype,
                output,
            } => {
                launch_ensembl_list_species(*release, datatype, output)?;
            }
        },
        Commands::Uniprot(sub) => match sub {
            ModUniprot::Query {
                search_terms,
                freeform,
                taxon,
                output,
            } => {
                launch_uniprot_query(search_terms, *freeform, taxon, output)?;
            }
        },
        Commands::Ncbi(sub) => match sub {
            ModNcbi::Taxons {
                query,
                limit,
                output,
            } => {
                launch_ncbi_taxons(query, *limit, output)?;
            }
            ModNcbi::QueryIds { ids, output } => {
                launch_ncbi_query_ids(ids, output)?;
            }
            ModNcbi::QuerySymbols {
                symbols,
                taxon_id,
                output,
            } => {
                launch_ncbi_query_symbols(symbols, *taxon_id, output)?;
            }
        },
        Commands::Pdb(sub) => match sub {
            ModPdb::Structure {
                pdb_id,
                header_only,
                format,
                output,
            } => {
                launch_pdb_structure(pdb_id, *header_only, format, output)?;
            }
            ModPdb::Info {
                pdb_id,
                resource,
                identifier,
                output,
            } => {
                launch_pdb_resource(pdb_id, resource, identifier, output)?;
            }
        },
        Commands::Ucsc(sub) => match sub {
            ModUcsc::Blat {
                sequence,
                seqtype,
                db_name,
                output,
            } => {
                launch_ucsc_blat(sequence, seqtype, db_name, output)?;
            }
        },
        Commands::Seq {
            search_terms,
            translate,
            species,
            output,
        } => {
            launch_seq(search_terms, *translate, species, output)?;
        }
        Commands::Blast {
            query,
            program,
            database,
            limit,
            expect,
            low_comp_filter,
            megablast,
            output,
        } => {
            launch_blast(
                &query,
                &program,
                &database,
                *limit,
                *expect,
                *low_comp_filter,
                *megablast,
                output,
            )?;
        }
        Commands::Autocomplete { shell } => print_completions(*shell, &mut Cli::command()),
    };

    Ok(())
}
