use std::{collections::HashMap, thread};

use crossbeam::channel::{unbounded, Receiver, Sender};

use crate::{drone::{Drone, DroneImplement}, routing::{packet::Packet, types::NodeId}, SimulationControllerCommand};

use super::config_parsing::{parse_config, InitConfig};

pub struct NetworkInitializer {
    config: InitConfig,
    packet_channels: HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
    sc_channels: HashMap<NodeId, (Sender<SimulationControllerCommand>, Receiver<SimulationControllerCommand>)>,
}

impl NetworkInitializer {
    pub fn new(config_path: String) -> NetworkInitializer {
        NetworkInitializer {
            config: parse_config(config_path),
            packet_channels: HashMap::new(),
            sc_channels: HashMap::new(),
        }
    }

    pub fn init_network(&mut self) {
        //create 3 different version since we might want the simulation controller channels to depend on node type
        for drone in self.config.drone.iter() {
            //create unbounded channel for drones
            self.packet_channels.insert(drone.id, unbounded::<Packet>());
            self.sc_channels.insert(drone.id, unbounded::<SimulationControllerCommand>());
        }

        for drone in self.config.drone.iter() {

            //clones all the sender channels for the connected drones
            let mut sender_channels: HashMap<NodeId, Sender<Packet>> = HashMap::new();

            for connected_drone in drone.connected_drone_ids.iter() {
                sender_channels.insert(*connected_drone, self.packet_channels.get(connected_drone).unwrap().0.clone());
            }
            
            let packet_receiver = self.packet_channels.get(&drone.id).unwrap().1.clone();
            let command_receiver = self.sc_channels.get(&drone.id).unwrap().1.clone();
            let command_send = self.sc_channels.get(&drone.id).unwrap().0.clone();

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
    }

    //just for testing purposes
    pub fn get_send_channel(& self, drone_id: NodeId) -> &Sender<Packet> {
        &self.packet_channels.get(&drone_id).unwrap().0
    }

}
