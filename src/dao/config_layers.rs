use crate::dao::DatabaseConnect;
use crate::schema::config_layers;
use diesel::RunQueryDsl;

#[derive(Insertable, Debug)]
#[table_name = "config_layers"]
struct NewConfigLayer<'a> {
    configid: &'a u32,
    layersid: &'a u32,
}
pub async fn insert_config_layers(
    config: &u32,
    layers: &Vec<u32>,
    connect: &DatabaseConnect,
) -> bool {
    let layers_size = layers.len();
    let data: Vec<NewConfigLayer> = layers
        .iter()
        .map(|x| NewConfigLayer {
            configid: config,
            layersid: x,
        })
        .collect();

    match diesel::insert_into(config_layers::table)
        .values(&data)
        .execute(connect)
    {
        Ok(num) => num.eq(&layers_size),
        Err(_) => {
            return false;
        }
    }
}
