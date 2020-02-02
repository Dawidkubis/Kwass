use std::collections::HashMap;

struct Scope {
	parent: Option<Scope>,
	vars: HashMap<>
}
