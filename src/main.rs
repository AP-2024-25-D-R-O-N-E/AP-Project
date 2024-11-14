//import modules
mod routing;
mod initializer;
mod drone;

use std::{collections::HashMap, fs, thread::{self, sleep}, time::Duration};

use crossbeam::channel::{unbounded, Receiver, Sender};
use drone::{Drone, DroneImplement};
// use 'use' for easier naming
use routing::{packet::{self, Ack, Packet, SourceRoutingHeader}, types::NodeId};
use initializer::{config_parsing::InitConfig, network_initializer::{self, NetworkInitializer}};

struct SimulationControllerCommand {
    //temporary
}

fn main() {
    
    let mut network_initializer = NetworkInitializer::new("src/config.toml".to_string());

    network_initializer.init_network();

    //* just some testing */

    let ack = Ack::new(0);

    let packet = Packet {
        pack_type: packet::PacketType::Ack(ack),
        routing_header: SourceRoutingHeader {
            hops: vec![0, 1, 2]
        },
        session_id: 0,
    };

    sleep(Duration::from_secs(1));
    let _ = network_initializer.get_send_channel(0).send(packet);

    let ack = Ack::new(5);

    let packet = Packet {
        pack_type: packet::PacketType::Ack(ack),
        routing_header: SourceRoutingHeader {
            hops: vec![0, 1, 2]
        },
        session_id: 0,
    };
    
    sleep(Duration::from_secs(5));
    let _ = network_initializer.get_send_channel(1).send(packet);

    // was simply testing

}
