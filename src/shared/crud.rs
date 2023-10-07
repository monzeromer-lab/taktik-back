use sea_orm::{*, sea_query::IntoCondition};
use std::convert::TryInto;

pub struct CRUD;


// Define the newtype wrapper for the primary key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MyPrimaryKey(pub u64);

// Implement the required traits for the newtype wrapper
impl std::convert::From<u64> for MyPrimaryKey {
    fn from(value: u64) -> Self {
        MyPrimaryKey(value)
    }
}

impl std::convert::TryInto<usize> for MyPrimaryKey {
    type Error = ();

    fn try_into(self) -> Result<usize, Self::Error> {
        self.0.try_into().map_err(|_| ())
    }
}


impl CRUD {

    pub async fn create<Connection, Model, Data>(
        data: Data,
        connection: Option<&Connection>,
    ) -> Result<InsertResult<Model>, Box<dyn std::error::Error>>
    where
        Connection: sea_orm::ConnectionTrait,
        Model: EntityTrait + sea_orm::ActiveModelTrait,
        Data: sea_orm::ActiveModelTrait + IntoActiveModel<Model>,
    {
        let connection: &Connection = connection.ok_or("Connection not provided")?;
    
        let inserted_model: InsertResult<Model> = Model::Entity::insert(data.into_active_model()).exec(connection).await.unwrap();
    
        Ok(inserted_model)
    }
    
    pub async fn read<Connection, Model, Data>(
        data: Option<Data>,
        connection: Option<&Connection>,
    ) -> Result<Vec<<Model as EntityTrait>::Model>, sea_orm::error::DbErr>
    where
        Connection: sea_orm::ConnectionTrait,
        Model: EntityTrait,
        Data: IntoCondition,
    {
        let query = match data {
            Some(filter) => Model::find().filter(filter),
            None => Model::find(),
        };
    
        query.all(connection.unwrap()).await
    }

    // pub async fn update<Connection, Model, UpdateWhere, Data>(
    //     update_at: UpdateWhere,
    //     data: Data,
    //     connection: Option<&Connection>,
    // ) -> Result<Model, Box<dyn std::error::Error>>
    // where
    //     Connection: sea_orm::ConnectionTrait,
    //     Model: EntityTrait,
    //     UpdateWhere: IntoCondition,
    //     Data: serde::de::DeserializeOwned,
    // {

    //     let mut model = Model::find().filter(update_at).one(connection.unwrap()).await?;

    //     // Update the model fields with the new data
    //     model.insert(data);

    //     let connection = connection.unwrap();

    //     // Save the updated model
    //     let updated_model = model.save_changes(connection).await?;

    //     Ok(updated_model)
    // }
    
    pub async fn delete<Connection, Model>(
        model: Model,
        connection: Option<&Connection>,
        data: MyPrimaryKey,
    ) -> Result<usize, DbErr>
    where
        Connection: sea_orm::ConnectionTrait,
        Model: EntityTrait + sea_orm::ActiveModelTrait,
        <<Model as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType: std::convert::From<MyPrimaryKey>,
    {
        let connection: &Connection = connection.unwrap();
    
        // Use a pattern guard to handle the outcome of the `find_by_id()` function.
        match Model::find_by_id(data.clone()).one(connection).await? {
            orange if orange.is_some() => {
                // Delete the `Model` instance by calling the `delete_by_id()` function.
                let res: DeleteResult = Model::delete_by_id(data).exec(connection).await?;
    
                // Return the number of rows that were affected by the delete operation.
                Ok(res.rows_affected.try_into().unwrap())
            }
            _ => Err(DbErr::RecordNotFound("id".to_owned())),
        }
    }
    
    
    
}