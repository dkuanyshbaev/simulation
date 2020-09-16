table! {
    bindings (id) {
        id -> Integer,
        file_name -> Text,
        heaven_pin -> Integer,
        heaven_colour -> Text,
        cloud_pin -> Integer,
        cloud_colour -> Text,
        sun_pin -> Integer,
        sun_colour -> Text,
        wind_pin -> Integer,
        wind_colour -> Text,
        thunder_sound -> Text,
        thunder_colour -> Text,
        water_pin -> Integer,
        water_colour -> Text,
        mountain_sound -> Text,
        mountain_colour -> Text,
        earth_pin -> Integer,
        earth_colour -> Text,
        multiply -> Text,
        bias -> Text,
        threshold -> Text,
    }
}
allow_tables_to_appear_in_same_query!(bindings,);
