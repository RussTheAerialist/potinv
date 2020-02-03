use juniper::{graphql_object, GraphQLObject};
use super::datastore::Datastore;
use minimal_id::MinimalId;

#[derive(GraphQLObject, Debug)]
pub(crate) struct Clay {
}

#[derive(Debug)]
pub(crate) struct Piece {
    clay_id: MinimalId,
}

graphql_object!(Piece: Datastore |&self| {
    field clay(&executor) -> &Clay {
        let database = executor.context();

        database.clays.get(&self.clay_id).expect("Unable to find clay with Id")
    }
});
