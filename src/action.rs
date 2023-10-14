
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
	MoveRight,
	MoveUp,
	MoveLeft,
	MoveDown,

	Macro(Vec<Action>),
}