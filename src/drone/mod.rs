use std::collections::HashMap;

use crossbeam::{channel::{Receiver, Sender}, select};

use crate::{routing::{packet::{NodeType, Packet, PacketType, Query, QueryResult, SourceRoutingHeader}, types::NodeId}, SimulationControllerCommand};

#[derive(Debug)]
pub struct Drone {
    drone_id: NodeId,
    sim_contr_send: Sender<SimulationControllerCommand>,   //Not packet.
    sim_contr_recv: Receiver<SimulationControllerCommand>, //Not packet.
    packet_send: HashMap<NodeId, Sender<Packet>>, //All the sender to other nodes.
    packet_recv: Receiver<Packet>, //This drone receiver, that will be linked to a sender given to every other drone.
    pdr: u8,                       //Would keep it in % to occupy less space, but could be f32.
    floods_tracker: HashMap<NodeId, u64> //not in the specification yet but will likely be needed to handle the Network Discovery Protocol
}

pub trait DroneImplement {
    fn new_drone(
        id: NodeId,
        scs: Sender<SimulationControllerCommand>,
        scr: Receiver<SimulationControllerCommand>,
        ps: HashMap<NodeId, Sender<Packet>>,
        pr: Receiver<Packet>,
        pdr: f32,
    ) -> Drone;
}

impl DroneImplement for Drone {
    fn new_drone(
        id: NodeId,
        scs: Sender<SimulationControllerCommand>,
        scr: Receiver<SimulationControllerCommand>,
        ps: HashMap<NodeId, Sender<Packet>>,
        pr: Receiver<Packet>,
        pdr: f32,
    ) -> Drone {
        Drone {
            drone_id: id,
            sim_contr_send: scs,
            sim_contr_recv: scr,
            packet_send: ps,
            packet_recv: pr,
            pdr: (pdr * 100.0) as u8,
            floods_tracker: HashMap::new(),
        }
    }
}

impl Drone {
    pub fn run(&mut self) {
        loop {
            select! {
                recv(self.packet_recv) -> packet_res => {
                    if let Ok(packet) = packet_res {
                    // each match branch may call a function to handle it to make it more readable

                        //temporary and just for testing
                        println!("received packet at drone {}", self.drone_id);
                        self.forward_packet(packet);

                        // match packet.pack_type {
                        //     PacketType::Nack(nack) => todo!(),
                        //     PacketType::Ack(ack) => todo!(),
                        //     PacketType::MsgFragment(fragment) => todo!(),
                        //     PacketType::Query(query) => todo!(),
                        //     PacketType::QueryResult(query_result) => todo!(),
                        // }
                    }
                },
                recv(self.sim_contr_recv) -> command_res => {
                    if let Ok(command) = command_res {
                        //handle the simulation controller's command
                    }
                }
            }
        }
    }

    fn forward_packet(&self, packet: Packet) {
        let current_pos = packet.routing_header.hops.iter().position(|&x| x == self.drone_id);
        if let Some(index) = current_pos {
            let next_hop = packet.routing_header.hops.get(index + 1);

            if let Some(next_node) = next_hop {

                let next_send = self.packet_send.get(next_node);

                if let Some(send_channel) = next_send {

                    let res = send_channel.send(packet);

                    if res.is_err() {
                        //means there was an error with the channel (crashed drone), should spawn error
                    }

                    // just testing
                    println!("packet sent from {}", self.drone_id);

                } else {
                    //this means the channel isn't connected, should spawn error
                }

            }
        }
    }

}