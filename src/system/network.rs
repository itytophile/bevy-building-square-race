use bevy::prelude::*;
use bevy_networking_turbulence::{NetworkEvent, NetworkResource, Packet};
use std::net::SocketAddr;

const SERVER_PORT: u16 = 14191;

pub fn connect_to_server(mut net: ResMut<NetworkResource>) {
    let ip_address =
        bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    let server_address = SocketAddr::new(ip_address, SERVER_PORT);

    net.connect(server_address);
    info!("Client connected");
}

pub fn send_packets(mut net: ResMut<NetworkResource>) {
    net.broadcast(Packet::from("PING"));
    info!("PING");
}

pub fn handle_packets(mut reader: EventReader<NetworkEvent>) {
    for event in reader.iter() {
        match event {
            NetworkEvent::Packet(handle, packet) => {
                let message = String::from_utf8_lossy(packet);
                info!("Got packet on [{}]: {}", handle, message);
            }
            event => info!("{event:?} received!"),
        }
    }
}
