pub struct CRUD;

use sea_orm::{*, sea_query::IntoCondition};


impl CRUD {
    pub async fn create<Connection: ConnectionTrait, Model: EntityTrait, Data>(data: Data){

    }

    // pub async fn read<DBConnection: ConnectionTrait, Model: EntityTrait, Data>(data: Option<Box<Data>>, connection: Option<DBConnection>) -> Vec<Model>{
    //     let result: Result<Vec<Model>, DbErr> = Model::find().all(Some(&connection.unwrap()).unwrap()).await.map_err(|_err| ());
    //     result
    // }
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

    pub async fn update<Connection: ConnectionTrait, Model: EntityTrait, UpdateWhere, Data>(update_at: UpdateWhere, data: Data){

    }

    pub async fn delete<Connection: ConnectionTrait, Model: EntityTrait,  DeleteWhere,>(data: DeleteWhere){

    }
}