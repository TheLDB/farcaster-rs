use crate::{
    constants::api_root::API_ROOT, types::auth::secret::Secret, types::v2::user::UserRoot,
    Farcaster,
};

impl Farcaster {
    pub async fn test_user(
        secret: Secret,
        fid: i64,
    ) -> Result<UserRoot, Box<dyn std::error::Error>> {
        let user_reqwest = reqwest::Client::new()
            .get(format!("{}/v2/user?fid={}", API_ROOT, fid))
            .header("Content-Type", "application/json")
            .header("Authorization", secret.result.token.secret)
            .send()
            .await?
            .text()
            .await?;

        let user: UserRoot = serde_json::from_str(&user_reqwest)?;
        Ok(user)
    }
}

// {
//     "result": {
//       "user": {
//         "fid": 123,
//         "username": "janedoe",
//         "displayName": "Jane Doe",
//         "avatar": {
//           "url": "https://example.com/foo.jpg",
//           "isVerified": true
//         },
//         "followerCount": 2423,
//         "followingCount": 328,
//         "isViewerFollowing": false,
//         "isFollowingViewer": false,
//         "profile": {
//           "bio": {
//             "text": "I like @farcaster",
//             "mentions": [
//               {
//                 "avatar": {
//                   "url": "https://explorer.farcaster.xyz/farcaster.png",
//                   "isVerified": false
//                 },
//                 "fid": 1,
//                 "username": "farcaster",
//                 "displayName": "Farcaster"
//               }
//             ]
//           }
//         }
//       }
//     }
//   }
