/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use super::{AccountCheckPayload, AccountCheckResp};
use crate::errors::*;
use crate::AppData;

#[my_codegen::post(path = "crate::V1_API_ROUTES.account.username_exists")]
async fn username_exists(
    payload: web::Json<AccountCheckPayload>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    let resp = runners::username_exists(&payload, &data).await?;
    Ok(HttpResponse::Ok().json(resp))
}

pub mod runners {
    use super::*;

    pub async fn username_exists(
        payload: &AccountCheckPayload,
        data: &AppData,
    ) -> ServiceResult<AccountCheckResp> {
        let exists = data.db.username_exists(&payload.val).await?;

        Ok(AccountCheckResp { exists })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Username {
    pub username: String,
}

/// update username
#[my_codegen::post(
    path = "crate::V1_API_ROUTES.account.update_username",
    wrap = "crate::api::v1::get_middleware()"
)]
async fn set_username(
    id: Identity,
    payload: web::Json<Username>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    let processed_uname = data.creds.username(&payload.username)?;

    data.db.update_username(&username, &processed_uname).await?;

    id.forget();
    id.remember(processed_uname);

    Ok(HttpResponse::Ok())
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(username_exists);
    cfg.service(set_username);
}
