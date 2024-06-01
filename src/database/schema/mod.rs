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

use prost_types::Timestamp;
use rbatis::rbdc::DateTime;

pub mod feedback;

pub fn date_time_to_timestamp(date_time: DateTime) -> Timestamp {
    Timestamp::date_time(
        date_time.year().into(),
        date_time.mon(),
        date_time.day(),
        date_time.hour(),
        date_time.minute(),
        date_time.sec(),
    )
    .unwrap()
}

#[macro_export]
macro_rules! to_date_time {
    ($ident:expr) => {{
        DateTime::from_timestamp($ident.unwrap().seconds)
    }};
}

#[macro_export()]
macro_rules! save_as_json {
    ($struct:path, $ident:ident) => {
        paste! {
            fn [<serialize_ $ident>]<S>(sub: &$struct, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let json_string = serde_json::to_string(sub).map_err(|error| serde::ser::Error::custom(format!("failed to serialize {} as json: {}", stringify!($struct), error)))?;
                serializer.serialize_str(&json_string)
            }

            fn [<deserialize_ $ident>]<'de, D>(deserializer: D) -> std::result::Result<$struct, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let json_string = String::deserialize(deserializer)?;
                serde_json::from_str(&json_string).map_err(|error| serde::de::Error::custom(format!("failed to deserialize {} from json: {}", stringify!($struct), error)))
            }
        }
    };
}
