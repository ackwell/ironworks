use juniper::graphql_value;

#[derive(juniper::GraphQLEnum, Clone, Copy)]
enum Episode {
	NewHope,
	Empire,
	Jedi,
}

struct Context(Episode);
impl juniper::Context for Context {}

struct Query;
#[juniper::graphql_object(context = Context)]
impl Query {
	fn favouriteEpisode(context: &Context) -> juniper::FieldResult<Episode> {
		return Ok(context.0);
	}
}

fn main() {
	let schema: juniper::RootNode<Query, juniper::EmptyMutation<Context>, juniper::EmptySubscription<Context>> = juniper::RootNode::new_with_info(
		Query,
		juniper::EmptyMutation::new(),
		juniper::EmptySubscription::new(),
		(),
		(),
		(),
	);

	let context = Context(Episode::NewHope);

	let (res, _errors) = juniper::execute_sync(
		"query { favouriteEpisode }", 
		None, 
		&schema, 
		&juniper::Variables::new(), 
		&context,
	)
		.unwrap();

	assert_eq!(
		res,
		graphql_value!({ "favouriteEpisode": "NEW_HOPE" })
	)
}
