//Holochain data types
#[derive(Serialize, Deserialize, Debug)]
pub struct ExpressionPost { 
    pub expression_type: ExpressionTypes,
    pub expression: Expression
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Expression {
    LongForm{
        title: String,
        body: String
    },
    ShortForm{
        background: String,
        body: String
    },
    PhotoForm{
        image: String,
        caption: String
    },
    EventForm{
        title: String,
        date: String,
        location: String,
        details: String
    },
    BulletForm{
        title: String,
        bullets: Vec<String>
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ExpressionTypes {
    LongForm,
    ShortForm,
    PhotoForm,
    EventForm,
    BulletForm
}

//Holochain request data types
#[derive(Debug, Serialize, Deserialize)]
pub enum QueryTarget{
    ExpressionPost,
    User
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QueryOptions {
    FilterPopular,
    FilterNew,
    FilterOld
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QueryType {
    And,
    Or
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryExpressionRequestData {
    pub perspective: String, 
    pub attributes: Vec<String>, 
    pub query_options: QueryOptions, 
    pub target_type: QueryTarget, 
    pub query_type: QueryType, 
    pub dos: u32, 
    pub seed: String, 
    pub resonations: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetExpressionRequestData {
    pub expression: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostExpressionRequestData {
    pub expression: ExpressionPost, 
    pub attributes: Vec<String>, 
    pub context: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostCommentExpressionRequestData {

}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostResonationRequestData {

}