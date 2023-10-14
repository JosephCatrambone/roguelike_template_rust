use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
	MoveRight,
	MoveUp,
	MoveLeft,
	MoveDown,

	Macro(Vec<Action>),
}