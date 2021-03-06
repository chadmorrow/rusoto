#![cfg(feature = "appmesh")]

extern crate rusoto_appmesh;
extern crate rusoto_core;

use rusoto_appmesh::{AppMesh, AppMeshClient, ListMeshesInput};
use rusoto_core::Region;

#[test]
fn main() {
    let appmesh = AppMeshClient::new(Region::UsEast1);

    match appmesh.list_meshes(ListMeshesInput::default()).sync() {
        Ok(response) => {
            for mesh_ref in response.meshes {
                println!("arn -> {:?}", mesh_ref.arn);
            }
        }
        Err(err) => {
            panic!("Error listing meshes {:#?}", err);
        }
    }
}
