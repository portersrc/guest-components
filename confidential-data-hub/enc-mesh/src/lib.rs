//
// SPDX-License-Identifier: Apache-2.0
//

pub mod error;
pub mod enc_mesh;
pub use error::*;
use crate::enc_mesh::nebula::NebulaMesh;


pub async fn set_up(pod_name: String,
                    lighthouse_pub_ip: String) -> Result<()> {
    let nm: NebulaMesh = NebulaMesh {};
    nm.set_up(pod_name, lighthouse_pub_ip).await?;
    Ok(())
}
