// (***********************************************************************)
// (*    Grew - a Graph Rewriting tool dedicated to NLP applications      *)
// (*                                                                     *)
// (*    Copyright 2011-2024 Inria, Université de Lorraine                *)
// (*                                                                     *)
// (*    Webpage: https://grew.fr                                         *)
// (*    License: CeCILL (see LICENSE folder or "http://www.cecill.info") *)
// (*    Authors: see AUTHORS file                                        *)
// (***********************************************************************)
//
// open Conll

use conll::{Conll_columns, Conll_config};

#[derive(Debug, Clone)]
pub enum Output {
    Conll(Conll_columns),
    Dot,
    Json,
    Multi_json,
    Tsv,
}
#[derive(Debug, Clone)]
pub struct CliArgs {
    pub quiet: bool,
    pub verbose: bool,
    pub subcommand: Option<String>,
    pub grs: Option<String>,
    pub dep_dir: Option<String>,
    pub output: Output,
    pub input_data: Vec<String>,
    pub output_data: Option<String>,
    pub strat: String,
    pub timeout: Option<f32>,
    pub requests: Vec<String>,
    pub html: bool,
    pub clustering: Vec<String>,
    pub config: Conll_config,
    pub force: bool,
    pub text_from_tokens: bool,
    pub anonymous_args: Vec<String>,
}

impl Default for CliArgs {
    fn default() -> Self {
        CliArgs {
            quiet: false,
            verbose: false,
            subcommand: None,
            grs: None,
            dep_dir: None,
            output: Output::Conll(Conll_columns::default()),
            input_data: Vec::default(),
            output_data: None,
            strat: "main".to_string(),
            timeout: None,
            requests: Vec::default(),
            html: false,
            clustering: Vec::default(),
            config: Conll_config::ud(),
            force: false,
            text_from_tokens: false,
            anonymous_args: Vec::default(),
        }
    }
}

//
//
// let vars = ["CORPUSBANK"; "UDTOOLS"; "SUDTOOLS"; "SUDVALIDATION"]
// let (env: (string * string) list ref) =
//   let init_env =
//     List.fold_left
//       (fun acc k ->
//         match Sys.getenv_opt k with
//         | None -> acc
//         | Some v -> (k,v)::acc
//       ) [] vars in
//   ref (init_env)
// let setenv  k v = env := (k,v) :: List.remove_assoc k !env
// let getenv_opt k = List.assoc_opt k !env
