//import modules
mod routing;
mod initializer;
mod drone;

use std::{collections::HashMap, fs, thread::{self, sleep}, time::Duration};

use crossbeam::channel::{unbounded, Receiver, Sender};
use drone::{Drone, DroneImplement};
// use 'use' for easier naming
use routing::{packet::{self, Ack, Packet, SourceRoutingHeader}, types::NodeId};
use initializer::InitConfig;

struct SimulationControllerCommand {
    //temporary
}

fn main() {
    let config_data = fs::read_to_string("src/config.toml").expect("Unable to read config file");

    let config: InitConfig = toml::from_str(&config_data).expect("Unable to parse TOML");

    // println!("{:?}", config);

    let mut packet_channels: Vec<(Sender<Packet>, Receiver<Packet>)> = Vec::new();
    //simulation controller channel
    let mut sc_channels: Vec<(Sender<SimulationControllerCommand>, Receiver<SimulationControllerCommand>)> = Vec::new();

    //create 3 different version since we might want the simulation controller channels to depend on node type
    for _ in config.drone.iter() {
        //create unbounded channel for drones
        packet_channels.push(unbounded::<Packet>());
        sc_channels.push(unbounded::<SimulationControllerCommand>());
    }

	for drone in config.drone.iter() {

        //clones all the sender channels for the connected drones
        let mut sender_channels: HashMap<NodeId, Sender<Packet>> = HashMap::new();

        for connected_drone in drone.connected_drone_ids.iter() {
            sender_channels.insert(*connected_drone, packet_channels[*connected_drone as usize].0.clone());
        }
        
        let packet_receiver = packet_channels[drone.id as usize].1.clone();
        let command_receiver = sc_channels[drone.id as usize].1.clone();
        let command_send = sc_channels[drone.id as usize].0.clone();

		// since the thread::spawn function will take ownership of the values, we need to copy or clone them to not have problems with the Vec
        let drone_id: NodeId = drone.id;

        let pdr = drone.pdr as f32;

        thread::spawn(move || {

            let mut drone = Drone::new_drone(drone_id, command_send, command_receiver, sender_channels, packet_receiver, pdr);

            println!("my thread's drone: {:?}", drone);
            // run function is where the logic of the drone runs.
            drone.run();
        });
    }

    let ack = Ack::new(0);

    let packet = Packet {
        pack_type: packet::PacketType::Ack(ack),
        routing_header: SourceRoutingHeader {
            hops: vec![0, 1, 2]
        },
        session_id: 0,
    };

    sleep(Duration::from_secs(1));
    let _ = packet_channels[0].0.send(packet);

    let ack = Ack::new(5);

    let packet = Packet {
        pack_type: packet::PacketType::Ack(ack),
        routing_header: SourceRoutingHeader {
            hops: vec![0, 1, 2]
        },
        session_id: 0,
    };
    
    sleep(Duration::from_secs(5));
    let _ = packet_channels[0].0.send(packet);

}
