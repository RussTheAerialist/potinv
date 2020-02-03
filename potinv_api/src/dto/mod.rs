mod datastore;
mod types;

use juniper::{EmptyMutation, RootNode, Context, GraphQLObject};
use self::datastore::Datastore;

#[derive(Debug)]
pub(crate) struct Query;
#[juniper::object(Context=Datastore)]
impl Query {
}

impl Context for Datastore {
}

pub(crate) type Schema = RootNode<'static, Query, EmptyMutation<Datastore>>;

pub(crate) fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::<Datastore>::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use juniper::Variables;

    #[test]
    fn can_query() {
        let q = r#"{ __schema { types { name } } } "#;
        let datastore = Datastore::default();
        let schema = schema();
        let (value, errors) = juniper::execute(q, None, &schema, &Variables::new(), &datastore).unwrap();
        let actual = value.as_object_value().unwrap().get_field_value("__schema").unwrap().as_object_value().unwrap();
        let actual = actual.get_field_value("types").unwrap().as_list_value().unwrap();
        actual.iter()
            .map(|o| o.as_object_value().unwrap())
            .flat_map(|o| o.iter())
            .for_each(|(s,v)| println!("{:?}={:?}", s, v));
        println!("errors={:?}", errors);
        assert!(false);
    }
}
