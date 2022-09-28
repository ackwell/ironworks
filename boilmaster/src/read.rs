use ironworks::excel;
use ironworks_schema as schema;

// TODO: need some representation of filtering for this, preferably that will be constructable from reference filters, gql queries, and a get request for rest
// TODO: this shouldn't return a string, i don't think. need some arbitrary nested format (nested dicts?) that can be translated depending on what format we're using
pub fn read_sheet(sheet: &schema::Sheet, row: &excel::Row) -> String {
	if sheet.order != schema::Order::Index {
		todo!("sheet schema {:?} order", sheet.order);
	}

	read_node(0, &sheet.node, row)
}

fn read_node(index: u32, node: &schema::Node, row: &excel::Row) -> String {
	match node {
		schema::Node::Scalar => read_scalar(index, row),
		schema::Node::Struct(definition) => read_struct(index, definition, row),
		node => format!("TODO FORMAT {node:?}"),
	}
}

fn read_scalar(index: u32, row: &excel::Row) -> String {
	format!("{:?}", row.field(index.try_into().unwrap()))
}

fn read_struct(index: u32, definition: &[(String, schema::Node)], row: &excel::Row) -> String {
	definition
		.iter()
		.scan(index, |index, (key, node)| {
			// TODO: this is wasteful, given it's going to recurse every child node to find the size - is that a problem? probably?
			let result = read_node(*index, node, row);
			*index += node.size();
			Some(format!("{key}: {}\n", result))
		})
		.collect::<String>()
}
