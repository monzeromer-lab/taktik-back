pub struct CRUD;

use sea_orm::*;
use sea_orm::entity::*;


impl CRUD {
    pub async fn create<Connection: ConnectionTrait, Model: EntityTrait, Data>(data: Data){

    }

    pub async fn read<DBConnection: ConnectionTrait, Model: EntityTrait, Data>(data: Option<Box<Data>>, connection: Option<DBConnection>){
        Model::find().all(Some(&connection.unwrap()).unwrap()).await;
    }

    pub async fn update<Connection: ConnectionTrait, Model: EntityTrait, UpdateWhere, Data>(update_at: UpdateWhere, data: Data){

    }

    pub async fn delete<Connection: ConnectionTrait, Model: EntityTrait,  DeleteWhere,>(data: DeleteWhere){

    }
}