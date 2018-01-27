use raft::message::Message;
use raft::server::Server;

#[derive(Debug)]
pub struct Log {
    message: Message,
    data: Vec<u8>
}


#[derive(Debug)]
pub enum State {
    Follower(Option<u8>),
    Candidate(),
    Leader(),
}

#[derive(Debug)]
pub struct Raft {
    pub state: State,
    pub term: u32,
    pub id: u8,
    pub log: Vec<Log>,
    pub timeout: u64,
    pub server: Server,
}

impl Raft {
    pub fn new(id: u8) -> Raft {
        Raft {
            state: State::Follower(None),
            id,
            term: 0,
            log: Vec::new(),
            timeout: 750,
            server: Server::new(id),
        }
    }

    pub fn init(&mut self) {
        self.server.listen();
    }

    pub fn become_candidate(&mut self) {
        self.state = State::Candidate();
    }

    pub fn handle_message(self, message: Message, data: Vec<u8>) -> Raft {
        return self;
    }
}