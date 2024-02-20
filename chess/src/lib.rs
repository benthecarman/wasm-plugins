use anyhow::anyhow;
use extism_pdk::*;
use pleco::bot_prelude::*;
use pleco::{BitMove, Board};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextMoveParams {
    pub board: String,
    pub algo: Option<String>,
    pub depth: Option<u16>,
}

#[plugin_fn]
pub fn calculate_next_move(Json(params): Json<NextMoveParams>) -> FnResult<String> {
    let board =
        Board::from_fen(&params.board).map_err(|e| anyhow!("Error parsing board: {e:?}"))?;

    let depth = params.depth.unwrap_or(1);
    let chess_move = match params.algo.as_deref() {
        None => IterativeSearcher::best_move(board.clone(), depth),
        Some("random") => RandomBot::best_move(board.clone(), depth),
        Some("minimax") => MiniMaxSearcher::best_move(board.clone(), depth),
        Some("parallel-minimax") => ParallelMiniMaxSearcher::best_move(board.clone(), depth),
        Some("jamboree") => JamboreeSearcher::best_move(board.clone(), depth),
        Some("iterative") => IterativeSearcher::best_move(board.clone(), depth),
        Some(algo) => return Err(anyhow!("Unknown algo: {algo}").into()),
    };

    if chess_move.get_raw() == 0 && board.checkmate() {
        let turn = board.turn().other_player().to_string();
        return Ok(json!({"winner": turn}).to_string());
    }

    let json = json!({"move": chess_move.stringify(), "data": chess_move.get_raw()});
    Ok(json.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyMoveParams {
    pub board: String,
    #[serde(rename = "move")]
    pub chess_move: u16,
}

#[plugin_fn]
pub fn apply_move(Json(params): Json<ApplyMoveParams>) -> FnResult<String> {
    let mut board =
        Board::from_fen(&params.board).map_err(|e| anyhow!("Error parsing board: {e:?}"))?;

    let bitmove = BitMove::new(params.chess_move);
    board.apply_move(bitmove);

    Ok(board.fen())
}
