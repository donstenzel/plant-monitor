use actix_web::{App, HttpServer, web};

mod handlers {
    use actix_web::{HttpResponse, web};

    use common::{db, models::*};

    macro_rules! crud {
        ($($base:ident)+) => {
            $( crud! { @ $base } )+
        };
        (@ $base:ident) => {
            paste::paste!{ crud! { @ [<$base:snake:lower s>] $base } }
        };
        (@ $module:ident $type:ident) => {
            pub mod $module {
                use super::*;

                pub async fn get_all() -> actix_web::Result<HttpResponse> {
                    let result = db::with_connection(&db::url(), db::$module::all)?;
                    Ok(HttpResponse::Ok().json(result))
                }

                pub async fn get(id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
                    let result = db::open(&db::url())
                        .and_then(|ref mut conn| db::$module::single(id.into_inner(), conn))?;
                    Ok(HttpResponse::Ok().json(result))
                }

                pub async fn delete(id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
                    let result = db::open(&db::url())
                        .and_then(|ref mut conn| db::$module::delete(id.into_inner(), conn))?;
                    Ok(HttpResponse::Ok().json(result))
                }

                pub async fn update(
                    id: web::Path<i32>,
                    update: web::Json<paste::paste!{ [<Update $type>] }>,
                ) -> actix_web::Result<HttpResponse> {
                    let result = db::open(&db::url()).and_then(|ref mut conn| {
                        db::$module::update(id.into_inner(), update.into_inner(), conn)
                    })?;
                    Ok(HttpResponse::Ok().json(result))
                }

                pub async fn create(new: web::Json<paste::paste! { [<New $type>] }>) -> actix_web::Result<HttpResponse> {
                    let result = db::open(&db::url())
                        .and_then(|ref mut conn| db::$module::create(new.into_inner(), conn))?;
                    Ok(HttpResponse::Ok().json(result))
                }
            }
        };
    }

    crud! { PlantType PotType Usage Measurement }
}
use handlers::*;

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Plant Types
            .route("/plant_types", web::post().to(plant_types::create))
            .route("/plant_types", web::get().to(plant_types::get_all))
            .route("/plant_types/{id}", web::get().to(plant_types::get))
            .route("/plant_types/{id}", web::put().to(plant_types::update))
            .route("/plant_types/{id}", web::delete().to(plant_types::delete))
            // Pot Types
            .route("/pot_types", web::post().to(pot_types::create))
            .route("/pot_types", web::get().to(pot_types::get_all))
            .route("/pot_types/{id}", web::get().to(pot_types::get))
            .route("/pot_types/{id}", web::put().to(pot_types::update))
            .route("/pot_types/{id}", web::delete().to(pot_types::delete))
            // Usages
            .route("/usages", web::post().to(usages::create))
            .route("/usages", web::get().to(usages::get_all))
            .route("/usages/{id}", web::get().to(usages::get))
            .route("/usages/{id}", web::put().to(usages::update))
            .route("/usages/{id}", web::delete().to(usages::delete))
            // Pot Types
            .route("/measurements", web::post().to(measurements::create))
            .route("/measurements", web::get().to(measurements::get_all))
            .route("/measurements/{id}", web::get().to(measurements::get))
            .route("/measurements/{id}", web::put().to(measurements::update))
            .route("/measurements/{id}", web::delete().to(measurements::delete))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
