use anyhow::anyhow;
use extism_pdk::*;
use pleco::bot_prelude::*;
use pleco::Board;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub board: String,
    pub algo: Option<String>,
    pub depth: Option<u16>,
}

#[plugin_fn]
pub fn calculate_next_move(Json(params): Json<Params>) -> FnResult<String> {
    let board =
        Board::from_fen(&params.board).map_err(|e| anyhow!("Error parsing board: {e:?}"))?;

    let depth = params.depth.unwrap_or(1);
    let chess_move = match params.algo.as_deref() {
        None => IterativeSearcher::best_move(board, depth),
        Some("random") => RandomBot::best_move(board, depth),
        Some("minimax") => MiniMaxSearcher::best_move(board, depth),
        Some("parallel-minimax") => ParallelMiniMaxSearcher::best_move(board, depth),
        Some("jamboree") => JamboreeSearcher::best_move(board, depth),
        Some("iterative") => IterativeSearcher::best_move(board, depth),
        Some(algo) => return Err(anyhow!("Unknown algo: {algo}").into()),
    };

    Ok(chess_move.stringify())
}
