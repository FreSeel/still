mod routes;

pub use routes::router;

#[cfg(test)]
pub(crate) use routes::router_with_images_dir;

#[cfg(test)]
mod tests;
