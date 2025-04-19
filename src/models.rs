
pub mod UserLogin {
    use sea_orm::entity::prelude::*;
    use sea_orm::{ActiveModelTrait, Database, DbErr, DeleteResult, EntityTrait, Set};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
    #[sea_orm(table_name = "user_login")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub username: String,
        
        pub password: String,
        pub ts: i32,
        
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}


