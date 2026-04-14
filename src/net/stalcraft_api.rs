use reqwest::blocking::Client;
use serde_json::Value; // Рекомендуется для динамических JSON
use serde::Deserialize;
use crate::config::{Config, DEMO_API};
use anyhow::{Context, Result};


#[derive(Deserialize)]
pub struct ClanMember {
    pub name: String,
}


type ClanMembersList = Vec<ClanMember>;


pub struct Requester{
    token: String,
    region: String,
    demo_api: bool
}

impl Requester {
    pub fn default() -> Requester {
        return Requester { token: Config::global().stalcraft_api_token.clone(), region: Config::global().stalcraft_region.clone(), demo_api: DEMO_API };
    }


    pub fn _get_data(&self, path: &str) -> Value {
        let client = Client::new();
        let domain = if self.demo_api { "d" } else { "e" };
        let url = format!("https://{}api.stalcraft.net/{}/{}", domain, self.region, path);

        let resp = client.get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .send()
            .expect("Ошибка запроса")
            .json::<Value>() // Используем Value для гибкости, как в Python r.json()
            .expect("Ошибка парсинга");

        resp
    }


    pub fn parse_data<T>(&self, path: &str) -> Result<T> 
    where 
        T: for<'de> serde::Deserialize<'de> 
    {
        let client = reqwest::blocking::Client::new();
        let domain = if self.demo_api { "d" } else { "e" };
        let url = format!("https://{}api.stalcraft.net/{}/{}", domain, self.region, path);

        // 2. Убираем expect, используем context и ?
        let response = client.get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .with_context(|| format!("Ошибка сети: не удалось связаться с API по адресу {}", url))?;

        // 3. Проверяем HTTP статус (важно для API!)
        if !response.status().is_success() {
            let status = response.status();
            let err_body = response.text().unwrap_or_else(|_| "пусто".into());
            anyhow::bail!("API вернул ошибку {}: {}", status, err_body);
        }

        let body = response.text().context("Не удалось прочитать тело ответа сервера")?;

        // 4. Парсим с подробным логом в случае неудачи
        serde_json::from_str::<T>(&body).with_context(|| {
            format!(
                "Ошибка парсинга JSON для типа {}.\nСырой ответ сервера:\n{}", 
                std::any::type_name::<T>(), 
                body
            )
        })
    }

    pub fn clan_members(&self) -> Result<Vec<String>> {
        let path = format!("clan/{}/members", Config::global().stalcraft_clan_id);
        
        Ok((self.parse_data::<ClanMembersList>(&path)?).into_iter().map(|m| m.name).collect())
    }
}