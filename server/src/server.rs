use tonic::{transport::Server, Request, Response, Status};

use palette::image_palette_server::{ImagePalette, ImagePaletteServer};
use palette::{AverageReply, AverageRequest};

pub mod palette {
    tonic::include_proto!("palette");
}

#[derive(Debug, Default)]
pub struct PaletteServer {}

#[tonic::async_trait]
impl ImagePalette for PaletteServer {
    async fn average_colour(
        &self,
        request: Request<AverageRequest>,
    ) -> Result<Response<AverageReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = AverageReply {
            success: true,
            output_path: Some("".to_string()),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = PaletteServer::default();

    Server::builder()
        .add_service(ImagePaletteServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
