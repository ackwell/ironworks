use juniper::graphql_value;

#[derive(juniper::GraphQLEnum, Clone, Copy)]
enum Episode {
	NewHope,
	Empire,
	Jedi,
}

struct Context(Episode);
impl juniper::Context for Context {}

struct Query {}

impl juniper::GraphQLType<juniper::DefaultScalarValue> for Query {
	fn name(_info: &()) -> Option<&'static str> {
		return Some("todo");
	}

	fn meta<'registry>(
		_info: &(),
		registry: &mut juniper::Registry<'registry>,
	) -> juniper::meta::MetaType<'registry>
	where
		juniper::DefaultScalarValue: 'registry,
	{
		let fields = &[registry.field::<&Episode>("favouriteEpisode", &())];

		return registry.build_object_type::<Query>(&(), fields).into_meta();
	}
}

impl juniper::GraphQLValue for Query {
	type Context = Context;
	type TypeInfo = ();

	fn type_name(&self, _info: &()) -> Option<&'static str> {
		return <Query as juniper::GraphQLType>::name(&());
	}

	fn resolve_field(
		&self,
		info: &(),
		_field_name: &str,
		_args: &juniper::Arguments,
		executor: &juniper::Executor<Context>,
	) -> juniper::ExecutionResult {
		let context = executor.context();
		return executor.resolve_with_ctx(info, &context.0);
	}
}

fn main() {
	let query = Query {};

	let schema: juniper::RootNode<
		Query,
		juniper::EmptyMutation<Context>,
		juniper::EmptySubscription<Context>,
	> = juniper::RootNode::new_with_info(
		query,
		juniper::EmptyMutation::new(),
		juniper::EmptySubscription::new(),
		(),
		(),
		(),
	);

	// ---

	let context = Context(Episode::NewHope);

	let (res, _errors) = juniper::execute_sync(
		"query { favouriteEpisode }",
		None,
		&schema,
		&juniper::Variables::new(),
		&context,
	)
	.unwrap();

	assert_eq!(res, graphql_value!({ "favouriteEpisode": "NEW_HOPE" }))
}
