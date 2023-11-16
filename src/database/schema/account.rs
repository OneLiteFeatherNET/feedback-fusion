/*
 * Copyright (c) 2023 OneLiteFeatherNET
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */

use rbatis::rbdc::DateTime;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, ToSchema)]
#[serde(untagged)]
pub enum Account {
    Internal(InternalAccount),
    External(ExternalAccount),
    Machine(MachineAccount),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Internal,
    External,
    Machine,
}

impl_select!(Account {select_by_id(id: &str) -> Option => "WHERE id = #{id} LIMIT 1"});
crud!(Account {});

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Getters, ToSchema)]
#[get = "pub"]
pub struct InternalAccount {
    id: String,
    username: String,
    password_hash: String,
    secret: String,
    totp: bool,
    /// always matches "internal"
    r#type: AccountType,
    updated_at: DateTime,
    created_at: DateTime,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Getters, ToSchema)]
#[get = "pub"]
pub struct ExternalAccount {
    id: String,
    username: String,
    /// "external"
    r#type: AccountType,
    updated_at: DateTime,
    created_at: DateTime,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Getters, ToSchema)]
#[get = "pub"]
pub struct MachineAccount {
    id: String,
    password_hash: String,
    /// "machine"
    r#type: AccountType,
    updated_at: DateTime,
    created_at: DateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! gain_id {
        ($variant:path, $var:expr) => {
            if let $variant(account) = $var {
                account.id.clone()
            } else {
                "".to_owned()
            }
        };
    }

    #[tokio::test]
    async fn test_parsing() {
        let connection = crate::tests::connect().await;

        let internal = Account::Internal(InternalAccount {
            id: nanoid!(),
            username: "internal".to_owned(),
            password_hash: "".to_owned(),
            secret: "".to_owned(),
            totp: false,
            r#type: AccountType::Internal,
            updated_at: DateTime::now(),
            created_at: DateTime::now(),
        });
        let external = Account::External(ExternalAccount {
            id: nanoid!(),
            username: "".to_owned(),
            r#type: AccountType::External,
            updated_at: DateTime::now(),
            created_at: DateTime::now(),
        });
        let machine = Account::Machine(MachineAccount {
            id: nanoid!(),
            password_hash: "".to_owned(),
            r#type: AccountType::Machine,
            updated_at: DateTime::now(),
            created_at: DateTime::now(),
        });

        Account::insert_batch(&connection, &[internal.clone(), external.clone(), machine.clone()], 3).await.unwrap();

        let internal_id = gain_id!(Account::Internal, internal.clone());
        let external_id = gain_id!(Account::External, external.clone());
        let machine_id = gain_id!(Account::Machine, machine.clone());

        let internal_parsed = Account::select_by_id(&connection, internal_id.as_str()).await.unwrap().unwrap();
        let external_parsed = Account::select_by_id(&connection, external_id.as_str()).await.unwrap().unwrap();
        let machine_parsed = Account::select_by_id(&connection, machine_id.as_str()).await.unwrap().unwrap();

        assert_eq!(internal, internal_parsed);
        assert_ne!(external, internal_parsed);
        assert_ne!(machine, internal_parsed);

        assert_eq!(external, external_parsed);
        assert_ne!(machine, external_parsed);

        assert_eq!(machine, machine_parsed);
    }
}

