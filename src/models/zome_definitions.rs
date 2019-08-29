use crate::models::{collection, config, expression, group, persepective, user};

//Holochain zomes
#[derive(Debug, Deserialize, Serialize)]
pub enum Zomes {
    Collection,
    Config,
    Expression,
    Group,
    Perspective,
    User,
}

//Holochain zome functions present in each zome
#[derive(Debug, Deserialize, Serialize)]
pub enum CollectionZomeFunctions{
    CreateDen,
    GetUserDens,
    IsCollectionOwner,
    CreateCollection,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ConfigZomeFunctions{
    GetEnv,
    GetCurrentBitPrefix,
    UpdateBitPrefix,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ExpressionZomeFunctions{
    QueryExpression,
    GetExpression,
    PostExpression,
    PostCommentExpression,
    PostResonation,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GroupZomeFunctions{
    CreatePack,
    AddPackMember,
    AddMemberToGroup,
    RemoveGroupMember,
    GetGroupMembers,
    IsGroupOwner,
    IsGroupMember,
    GetUserPacks,
    GetUserMemberPacks,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PerspectiveZomeFunctions{
    CreatePerspective,
    AddUserToPerspective,
    GetPerspectiveUsers,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum UserZomeFunctions{
    CreateUser,
    GetUsernameFromAddress,
    GetUserProfileFromAddress,
    GetUserProfileByAgentAddress,
    GetUserUsernameByAgentAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AllZomeFunctions{
    CreateDen,
    GetUserDens,
    IsCollectionOwner,
    CreateCollection,

    GetEnv,
    GetCurrentBitPrefix,
    UpdateBitPrefix,

    QueryExpression,
    GetExpression,
    PostExpression,
    PostCommentExpression,
    PostResonation,

    CreatePack,
    AddPackMember,
    AddMemberToGroup,
    RemoveGroupMember,
    GetGroupMembers,
    IsGroupOwner,
    IsGroupMember,
    GetUserPacks,
    GetUserMemberPacks,

    CreatePerspective,
    AddUserToPerspective,
    GetPerspectiveUsers,

    CreateUser,
    GetUsernameFromAddress,
    GetUserProfileFromAddress,
    GetUserProfileByAgentAddress,
    GetUserUsernameByAgentAddress,
}

// #[derive(Debug, Deserialize, Serialize)]
// pub enum AllZomeFunctionsRequestData{
//     collection::CreateDen,
//     collection::GetUserDens,
//     collection::IsCollectionOwner,
//     collection::CreateCollection,

//     config::GetEnv,
//     config::GetCurrentBitPrefix,
//     config::UpdateBitPrefix,

//     expression::QueryExpression,
//     expression::GetExpression,
//     expression::PostExpression,
//     expression::PostCommentExpression,
//     expression::PostResonation,

//     group::CreatePack,
//     group::AddPackMember,
//     group::AddMemberToGroup,
//     group::RemoveGroupMember,
//     group::GetGroupMembers,
//     group::IsGroupOwner,
//     group::IsGroupMember,
//     group::GetUserPacks,
//     group::GetUserMemberPacks,

//     perspective::CreatePerspective,
//     perspective::AddUserToPerspective,
//     perspective::GetPerspectiveUsers,
    
//     user::CreateUser,
//     user::GetUsernameFromAddress,
//     user::GetUserProfileFromAddress,
//     user::GetUserProfileByAgentAddress,
//     user::GetUserUsernameByAgentAddress,
// }
