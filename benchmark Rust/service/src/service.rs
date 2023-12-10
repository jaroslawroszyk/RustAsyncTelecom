use generated::ap::*;
use protobuf::Message;
use std::io::Write;
use sorted_vector_map::SortedVectorMap as HashMap;
use std::net::TcpStream;

fn send_serialized(msg: &Ap, mut writer: &TcpStream) {
    let mut buf: Vec<u8> = Vec::with_capacity(100);
    buf.push(msg.compute_size() as u8);
    let _ = msg.write_to_writer(&mut buf);
    let _ = writer.write(&buf);
}

pub struct UeService {
    ue_id_allocation: u32,
    cell_avg_data: HashMap<u32, u64>,
    ue_data_storage: HashMap<u32, (u64, u32)>,
}

impl UeService {
    pub fn new() -> Self {
        UeService {
            ue_id_allocation: 0,
            cell_avg_data: HashMap::new(),
            ue_data_storage: HashMap::new(),
        }
    }

    pub fn handle_message(&mut self, payload: &[u8], writer: &TcpStream) {
        match Ap::parse_from_bytes(payload) {
            Ok(message) => match message.msgtype {
                Some(ap::Msgtype::AddUserReq(ref msg)) => {
                    self.handle_add_user_req(msg, writer)
                }
                Some(ap::Msgtype::ModifyUserReq(ref msg)) => {
                    self.handle_modify_user_req(msg, writer)
                }
                Some(ap::Msgtype::ReleaseUserReq(ref msg)) => {
                    self.handle_release_user_req(msg, writer)
                }
                Some(ap::Msgtype::BulkReleaseReq(ref msg)) => {
                    self.handle_bulk_release_req(msg, writer)
                }
                _ => {
                    panic!("Unsupported msg type");
                }
            },
            Err(e) => {
                eprintln!("Error deserializing message: {e}");
            }
        }
    }

    fn handle_add_user_req(&mut self, req: &AddUserReq, writer: &TcpStream) {
        let mut resp_msg = Ap::new();
        let resp = resp_msg.mut_add_user_resp();
        resp.seq_iq = req.seq_iq;
        resp.set_user_id(self.add_ue_data(req.data, req.cell_id));

        send_serialized(&resp_msg, writer);
    }

    fn handle_modify_user_req(&mut self, req: &ModifyUserReq, writer: &TcpStream) {
        let mut resp_msg = Ap::new();
        let resp = resp_msg.mut_modify_user_resp();
        resp.user_id = req.user_id;

        if self.modify_ue_data(req.user_id, req.data).is_err() {
            resp.set_code(Error::UNKNOWN_USER_ID);
        } else {
            resp.set_data(req.data);
        }

        send_serialized(&resp_msg, writer);
    }

    fn handle_release_user_req(&mut self, req: &ReleaseUserReq, writer: &TcpStream) {
        let mut resp_msg = Ap::new();
        let resp = resp_msg.mut_release_user_resp();
        resp.user_id = req.user_id;

        self.remove_ue_data(resp.user_id);

        send_serialized(&resp_msg, writer);
    }

    fn handle_bulk_release_req(&mut self, req: &BulkReleaseReq, writer: &TcpStream) {
        let mut resp_msg = Ap::new();
        let resp = resp_msg.mut_bulk_release_resp();
        resp.cell_id = req.cell_id;

        self.remove_ue_by_cell(resp.cell_id);

        send_serialized(&resp_msg, writer);
    }

    fn allocate_user_id(&mut self) -> u32 {
        self.ue_id_allocation += 1;
        self.ue_id_allocation
    }

    fn add_ue_data(&mut self, value: u64, cell_id: u32) -> u32 {
        let key = self.allocate_user_id();
        if self.ue_data_storage.contains_key(&key) {
            panic!("Not supported");
        }

        self.ue_data_storage.insert(key, (value, cell_id));
        self.update_averages_of_data_in_cell(cell_id);
        key
    }

    fn modify_ue_data(&mut self, key: u32, value: u64) -> Result<u64, &'static str> {
        if let Some(existing_value) = self.ue_data_storage.get_mut(&key) {
            existing_value.0 = value;
            let cell_id = existing_value.1;
            self.update_averages_of_data_in_cell(cell_id);
            Ok(value)
        } else {
            Err("Key not found")
        }
    }

    fn remove_ue_data(&mut self, key: u32) {
        if let Some(user) = self.ue_data_storage.remove(&key) {
            self.update_averages_of_data_in_cell(user.1);
        }
    }

    fn remove_ue_by_cell(&mut self, cell_id: u32) {
        //SortedVectorMap hand written retain method        
        let mut ue_to_remove = Vec::with_capacity(10000);
        for (key, (_, stored_cell_id)) in self.ue_data_storage.iter() {
            if *stored_cell_id == cell_id {
                ue_to_remove.push(*key);
            }
        }
        for ue in ue_to_remove {
            self.ue_data_storage.remove(&ue);
        }
        
        println!(
            "Number of users in storage {}. Last allocated user_id {}",
            self.ue_data_storage.len(),
            self.ue_id_allocation
        );
    }

    fn update_averages_of_data_in_cell(&mut self, target_cell_id: u32) {
        let mut cnt = 0u32;
        let mut sum = 0u128;
        for &(data, cell_id) in self.ue_data_storage.values() {
            if cell_id == target_cell_id {
                cnt += 1;
                sum += data as u128;
            }
        }
        if cnt > 0 {
            let avg = (sum / cnt as u128) as u64;
            self.cell_avg_data.insert(target_cell_id, avg);
        }
    }
}