use sea_orm::{sea_query::IntoCondition, *};
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
        Model: ModelTrait + sea_orm::ActiveModelTrait,
        Data: sea_orm::ActiveModelTrait + sea_orm::ActiveModelTrait,
    {
        let connection: &Connection = connection.ok_or("Connection not provided")?;
        
        let inserted_model: InsertResult<Data> = Model::insert(data.into_active_model(), connection)
            // .exec(connection)
            .await
            .unwrap();

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

    pub async fn update<Connection, Model, UpdateWhere, Data>(
        update_at: UpdateWhere,
        data: Data,
        connection: Option<&Connection>,
        model_id: MyPrimaryKey,
    ) -> Result<UpdateResult, Box<dyn std::error::Error>>
    where
        Connection: sea_orm::ConnectionTrait,
        Model: EntityTrait + sea_orm::ActiveModelTrait,
        UpdateWhere: IntoCondition,
        Data: sea_orm::ActiveModelTrait<Entity = Model>,
        <<Model as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType:
            std::convert::From<MyPrimaryKey>,
    {
        let connection = connection.unwrap();
    
        let new_data = data.into_active_model();
    
        let new_doc = Model::update_many()
            .set(new_data)
            .filter(update_at)
            .exec(connection)
            .await?;
    
        Ok(new_doc)
    }
    

    pub async fn delete<Connection, Model>(
        connection: Option<&Connection>,
        data: MyPrimaryKey,
    ) -> Result<usize, DbErr>
    where
        Connection: sea_orm::ConnectionTrait,
        Model: EntityTrait + sea_orm::ActiveModelTrait,
        <<Model as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType:
            std::convert::From<MyPrimaryKey>,
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
